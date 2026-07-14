use crate::prelude::internal::*;
use std::collections::BTreeMap;

pub struct GtcCompilation<'project, 'backend, Backend: GtBackend + ?Sized> {
    /// Project to compile.
    project: &'project GtProject,

    /// Compiler backend to use for file system operations and diagnostics handling.
    backend: &'backend Backend,

    /// Count of errors encountered during compilation.
    errors_count: usize,

    /// Generated language modules mapped by their source path.
    meta_modules: BTreeMap<String, GtcMetaCompiledModule>,

    /// Resolved project and generated language paths.
    meta_paths: GtcMetaCompiledPaths,
}

impl<Backend: GtBackend + ?Sized> GtcCompilation<'_, '_, Backend> {
    pub fn new<'project, 'backend>(
        project: &'project GtProject,
        backend: &'backend Backend,
    ) -> GtcCompilation<'project, 'backend, Backend> {
        GtcCompilation {
            project,
            backend,
            errors_count: 0,
            meta_modules: BTreeMap::new(),
            meta_paths: GtcMetaCompiledPaths {
                src: project.paths().src.to_string(),
                dist: project.paths().dist.to_string(),
                ts: None,
                rs: None,
                py: None,
            },
        }
    }

    pub async fn compile(&mut self) -> Result<i32> {
        self.compile_langs(&[GtLang::Ts, GtLang::Py, GtLang::Rs])
            .await
    }

    pub async fn compile_langs(&mut self, langs: &[GtLang]) -> Result<i32> {
        let project = self.project;

        let project_diagnostics = project.as_final_diagnostics();
        self.handle_diagnostics(&project_diagnostics).await?;

        if langs.contains(&GtLang::Ts) {
            self.compile_project(&TsCompiler::new(project)).await?;
            self.run_lang_formatters(GtLang::Ts).await?;
        }

        if langs.contains(&GtLang::Py) {
            self.compile_project(&PyCompiler::new(project)).await?;
            self.run_lang_formatters(GtLang::Py).await?;
        }

        if langs.contains(&GtLang::Rs) {
            self.compile_project(&RsCompiler::new(project)).await?;
            self.run_lang_formatters(GtLang::Rs).await?;
        }

        self.finalize(&project.paths().dist).await
    }

    pub fn meta_modules(&self) -> Vec<GtcMetaCompiledModule> {
        self.meta_modules.values().cloned().collect()
    }

    pub fn meta_paths(&self) -> GtcMetaCompiledPaths {
        self.meta_paths.clone()
    }

    async fn run_lang_formatters(&mut self, lang: GtLang) -> Result<()> {
        if !self.project.lang_enabled(lang) {
            return Ok(());
        }

        let lang_config = self.project.config().lang(lang);
        let dist_path = self
            .project
            .paths()
            .dist
            .join_as_cwd_relative_path(&lang_config.dist_relative_pkg_path());
        let global_formatters = self.project.config().formatters.clone();
        let target_formatters = lang_config.common().formatters.clone();

        self.run_formatters(&global_formatters, &dist_path).await?;
        self.run_formatters(&target_formatters, &dist_path).await
    }

    async fn run_formatters(
        &mut self,
        formatters: &[GtpFormatter],
        path: &GtpCwdRelativePath,
    ) -> Result<()> {
        for formatter in formatters {
            if let Err(err) = self.backend.run_formatter(formatter, path).await {
                self.handle_diagnostics(&[GtDiagnostic::warning(format!(
                    "Failed to run formatter in `{path}`: {err}"
                ))])
                .await?;
            }
        }

        Ok(())
    }

    async fn compile_project<'project, Compiler: GtlCompiler<'project>>(
        &mut self,
        compiler: &Compiler,
    ) -> Result<()>
    where
        <<Compiler as GtlCompiler<'project>>::ProjectModule as GtlProjectModule>::LangConfig:
            'project,
    {
        match compiler.compile() {
            Ok(Some(dist)) => {
                collect_meta_paths(
                    &mut self.meta_paths,
                    compiler.lang(),
                    GtcMetaCompiledPathsLang {
                        pkg: compiler.config().pkg_dir_path().to_string(),
                        src: compiler.config().pkg_src_path().to_string(),
                    },
                );
                let GtlDist {
                    files,
                    modules,
                    diagnostics,
                } = dist;
                collect_meta_modules(&mut self.meta_modules, compiler.lang(), modules);

                let dist_diagnostics = diagnostics;
                self.handle_diagnostics(&dist_diagnostics).await?;

                let write_diagnostics = self.write_files(&files).await;
                self.handle_diagnostics(&write_diagnostics).await?;
            }

            Ok(None) => {}

            Err(err) => {
                self.handle_diagnostics(&[GtDiagnostic::error(format!("{err:?}"))])
                    .await?;
            }
        }

        Ok(())
    }

    async fn finalize(&mut self, dist_dir: &GtpDistDirPath) -> Result<i32> {
        let errors_count = self.errors_count;
        if errors_count > 0 {
            self.backend
                .report_diagnostic(&GtDiagnostic::warning(format!(
                    "Project generated to `{dist_dir}` with {errors_count} errors"
                )))
                .await?;

            return Ok(1);
        }

        self.backend
            .report_diagnostic(&GtDiagnostic::success(format!(
                "Project generated to `{dist_dir}`"
            )))
            .await?;

        Ok(0)
    }

    async fn handle_diagnostics(&mut self, diagnostics: &[GtDiagnostic]) -> Result<()> {
        self.errors_count += diagnostics
            .iter()
            .filter(|diagnostic| matches!(diagnostic.kind, GtDiagnosticKind::Error))
            .count();

        self.backend.report_diagnostics(diagnostics).await
    }

    async fn write_files(&self, files: &Vec<GtlDistFile>) -> Vec<GtDiagnostic> {
        let mut diagnostics = vec![];

        for file in files {
            let file_diagnostics = self.write_file(file).await;
            diagnostics.extend(file_diagnostics);
        }

        diagnostics
    }

    async fn write_file(&self, file: &GtlDistFile) -> Vec<GtDiagnostic> {
        let mut diagnostics = vec![];
        let path = &file.path();
        let source_code = file.source_code();

        let should_write = match file {
            GtlDistFile::Generated(_) => true,

            GtlDistFile::Error(error) => {
                // We only write the errored file if it doesn't exist in the file system, to avoid
                // overwriting existing files with errors.
                let file_exist_result = self.backend.file_exists(path.cwd_relative_path()).await;

                match file_exist_result {
                    Ok(false) => true,

                    Ok(true) | Err(_) => {
                        diagnostics.push(GtDiagnostic::warning(format!(
                            "Failed to write `{path}` to file system as it was generated with errors: {message}",
                            message = error.message
                        )));

                        if let Err(err) = file_exist_result {
                            diagnostics.push(GtDiagnostic::error(format!(
                                "Failed to check if `{path}` exists in file system: {err}"
                            )));
                        }

                        false
                    }
                }
            }
        };

        if should_write {
            let write_result = self
                .backend
                .write_file(path.cwd_relative_path(), source_code)
                .await;
            if let Err(err) = write_result {
                diagnostics.push(GtDiagnostic::error(format!(
                    "Failed to write `{path}` to file system: {err}"
                )));
            }
        }

        diagnostics
    }
}

fn collect_meta_modules(
    meta_modules: &mut BTreeMap<String, GtcMetaCompiledModule>,
    lang: GtLang,
    modules: Vec<GtlDistModule>,
) {
    for module in modules {
        let source = module.source_path.to_string();
        let target = module.target_path.to_string();
        let meta_module =
            meta_modules
                .entry(source.clone())
                .or_insert_with(|| GtcMetaCompiledModule {
                    source,
                    ts: None,
                    rs: None,
                    py: None,
                });

        match lang {
            GtLang::Ts => meta_module.ts = Some(target),
            GtLang::Rs => meta_module.rs = Some(target),
            GtLang::Py => meta_module.py = Some(target),
        }
    }
}

fn collect_meta_paths(
    paths: &mut GtcMetaCompiledPaths,
    lang: GtLang,
    lang_paths: GtcMetaCompiledPathsLang,
) {
    match lang {
        GtLang::Ts => paths.ts = Some(lang_paths),
        GtLang::Rs => paths.rs = Some(lang_paths),
        GtLang::Py => paths.py = Some(lang_paths),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn collects_and_merges_generated_modules_by_source() {
        let mut modules = BTreeMap::new();
        collect_meta_modules(
            &mut modules,
            GtLang::Ts,
            vec![dist_module("src/z.type", "dist/ts/z.ts")],
        );
        collect_meta_modules(
            &mut modules,
            GtLang::Rs,
            vec![
                dist_module("src/a.type", "dist/rs/a.rs"),
                dist_module("src/z.type", "dist/rs/z.rs"),
            ],
        );

        assert_eq!(
            modules.values().cloned().collect::<Vec<_>>(),
            vec![
                GtcMetaCompiledModule {
                    source: "src/a.type".into(),
                    ts: None,
                    rs: Some("dist/rs/a.rs".into()),
                    py: None,
                },
                GtcMetaCompiledModule {
                    source: "src/z.type".into(),
                    ts: Some("dist/ts/z.ts".into()),
                    rs: Some("dist/rs/z.rs".into()),
                    py: None,
                },
            ]
        );
    }

    #[test]
    fn collects_paths_for_compiled_languages() {
        let mut paths = GtcMetaCompiledPaths {
            src: "src".into(),
            dist: "dist".into(),
            ts: None,
            rs: None,
            py: None,
        };
        collect_meta_paths(
            &mut paths,
            GtLang::Ts,
            GtcMetaCompiledPathsLang {
                pkg: "dist/ts".into(),
                src: "dist/ts/src".into(),
            },
        );

        assert_eq!(
            paths,
            GtcMetaCompiledPaths {
                src: "src".into(),
                dist: "dist".into(),
                ts: Some(GtcMetaCompiledPathsLang {
                    pkg: "dist/ts".into(),
                    src: "dist/ts/src".into(),
                }),
                rs: None,
                py: None,
            }
        );
    }

    #[test]
    fn resolves_paths_only_for_compiled_languages() {
        let base_path: GtpCwdRelativeOrAbsoluteStringPath = ".".into();
        let backend = GtbSystem::new(&base_path).unwrap();
        let config_path = "../crate-genotype-lang-ts-project/examples/basic/genotype.toml".into();
        let project =
            block_on(backend.create_project_and_load_all_modules(Some(&config_path))).unwrap();
        let mut compilation = GtcCompilation::new(&project, &backend);

        block_on(compilation.compile_langs(&[GtLang::Ts])).unwrap();

        assert_eq!(
            compilation.meta_paths(),
            GtcMetaCompiledPaths {
                src: "../crate-genotype-lang-ts-project/examples/basic/src".into(),
                dist: "../crate-genotype-lang-ts-project/examples/basic/dist".into(),
                ts: Some(GtcMetaCompiledPathsLang {
                    pkg: "../crate-genotype-lang-ts-project/examples/basic/dist/ts".into(),
                    src: "../crate-genotype-lang-ts-project/examples/basic/dist/ts/src".into(),
                }),
                rs: None,
                py: None,
            }
        );
    }

    fn dist_module(source: &str, target: &str) -> GtlDistModule {
        GtlDistModule {
            source_path: source.into(),
            target_path: target.into(),
        }
    }
}
