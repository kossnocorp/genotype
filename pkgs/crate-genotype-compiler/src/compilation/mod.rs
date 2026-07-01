use crate::prelude::internal::*;

pub struct GtcCompilation<'project, 'backend, Runtime: GtcRuntime + ?Sized> {
    /// Project to compile.
    project: &'project GtProject,

    /// Compiler runtime to use for file system operations and diagnostics handling.
    runtime: &'backend Runtime,

    /// Count of errors encountered during compilation.
    errors_count: usize,
}

impl<Runtime: GtcRuntime + ?Sized> GtcCompilation<'_, '_, Runtime> {
    pub fn new<'project, 'backend>(
        project: &'project GtProject,
        runtime: &'backend Runtime,
    ) -> GtcCompilation<'project, 'backend, Runtime> {
        GtcCompilation {
            project,
            // backend,
            runtime,
            errors_count: 0,
        }
    }

    pub fn compile(&mut self) -> i32 {
        self.compile_langs(&[GtLang::Ts, GtLang::Py, GtLang::Rs])
    }

    pub fn compile_langs(&mut self, langs: &[GtLang]) -> i32 {
        let project = self.project;

        let project_diagnostics = project.as_final_diagnostics();
        self.handle_diagnostics(project_diagnostics);

        if langs.contains(&GtLang::Ts) {
            self.compile_project(&TsCompiler::new(project));
        }

        if langs.contains(&GtLang::Py) {
            self.compile_project(&PyCompiler::new(project));
        }

        if langs.contains(&GtLang::Rs) {
            self.compile_project(&RsCompiler::new(project));
        }

        self.finalize(&project.paths().dist)
    }

    fn compile_project<'project, Compiler: GtlCompiler<'project>>(&mut self, compiler: &Compiler)
    where
        <<Compiler as GtlCompiler<'project>>::ProjectModule as GtlProjectModule>::LangConfig:
            'project,
    {
        match compiler.compile() {
            Ok(Some(dist)) => {
                let dist_diagnostics = dist.diagnostics;
                self.handle_diagnostics(dist_diagnostics);

                let write_diagnostics = self.write_files(&dist.files);
                self.handle_diagnostics(write_diagnostics);
            }

            Ok(None) => {
                // Compiler is disabled, do nothing.
            }

            Err(err) => {
                self.handle_diagnostics(GtDiagnostic::error(format!("{err:?}")));
            }
        }
    }

    fn finalize(&mut self, dist_dir: &GtpDistDirPath) -> i32 {
        let errors_count = self.errors_count;
        if errors_count > 0 {
            self.runtime
                .report_diagnostic(&GtDiagnostic::warning(format!(
                    "Project generated to `{dist_dir}` with {errors_count} errors"
                )));

            return 1;
        }

        self.runtime
            .report_diagnostic(&GtDiagnostic::success(format!(
                "Project generated to `{dist_dir}`"
            )));

        0
    }

    fn handle_diagnostics<Diagnostics: Into<Vec<GtDiagnostic>>>(
        &mut self,
        diagnostics: Diagnostics,
    ) {
        let diagnostics = diagnostics.into();
        self.errors_count += diagnostics
            .iter()
            .filter(|diagnostic| matches!(diagnostic.kind, GtDiagnosticKind::Error))
            .count();

        self.runtime.report_diagnostics(&diagnostics);
    }

    fn write_files(&self, files: &Vec<GtlDistFile>) -> Vec<GtDiagnostic> {
        let mut diagnostics = vec![];

        for file in files {
            let file_diagnostics = self.write_file(file);
            diagnostics.extend(file_diagnostics);
        }

        diagnostics
    }

    fn write_file(&self, file: &GtlDistFile) -> Vec<GtDiagnostic> {
        let mut diagnostics = vec![];
        let path = &file.path();
        let source_code = file.source_code();

        let should_write = match file {
            GtlDistFile::Generated(_) => true,

            GtlDistFile::Error(error) => {
                // We only write the errored file if it doesn't exist in the file system, to avoid
                // overwriting existing files with errors.
                let file_exist_result = self.runtime.file_exists(path.cwd_relative_path());

                match file_exist_result {
                    // Write to file system if it doesn't exist
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
                .runtime
                .write_file(&path.cwd_relative_path(), source_code);
            if let Err(err) = write_result {
                diagnostics.push(GtDiagnostic::error(format!(
                    "Failed to write `{path}` to file system: {err}"
                )));
            }
        }

        diagnostics
    }
}
