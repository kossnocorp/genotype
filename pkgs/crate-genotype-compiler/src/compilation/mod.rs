use crate::prelude::internal::*;

pub struct GtcCompilation<'project, 'backend, Backend: GtBackend + ?Sized> {
    /// Project to compile.
    project: &'project GtProject,

    /// Compiler backend to use for file system operations and diagnostics handling.
    backend: &'backend Backend,

    /// Count of errors encountered during compilation.
    errors_count: usize,
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
                let dist_diagnostics = dist.diagnostics;
                self.handle_diagnostics(&dist_diagnostics).await?;

                let write_diagnostics = self.write_files(&dist.files).await;
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
                .write_file(&path.cwd_relative_path(), source_code)
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
