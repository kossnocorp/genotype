use crate::prelude::internal::*;

pub struct GtcCompilation<'project, 'backend> {
    /// Project to compile.
    project: &'project GtProject,

    /// Compiler runtime to use for file system operations and notice handling.
    backend: &'backend dyn GtcBackend,

    /// Count of errors encountered during compilation.
    errors_count: usize,
}

impl GtcCompilation<'_, '_> {
    pub fn new<'project, 'backend>(
        project: &'project GtProject,
        backend: &'backend dyn GtcBackend,
    ) -> GtcCompilation<'project, 'backend> {
        GtcCompilation {
            project,
            backend,
            errors_count: 0,
        }
    }

    pub fn compile(&mut self) -> i32 {
        self.compile_langs(&[GtLang::Ts, GtLang::Py, GtLang::Rs])
    }

    pub fn compile_langs(&mut self, langs: &[GtLang]) -> i32 {
        let project = self.project;

        let project_notices = project.as_final_notices();
        self.handle_notices(project_notices);

        if langs.contains(&GtLang::Ts) {
            self.compile_project(&TsCompiler::new(project));
        }

        if langs.contains(&GtLang::Py) {
            self.compile_project(&PyCompiler::new(project));
        }

        if langs.contains(&GtLang::Rs) {
            self.compile_project(&RsCompiler::new(project));
        }

        self.finalize(&project.paths.dist)
    }

    fn compile_project<'project, Compiler: GtlCompiler<'project>>(&mut self, compiler: &Compiler)
    where
        <<Compiler as GtlCompiler<'project>>::ProjectModule as GtlProjectModule>::LangConfig:
            'project,
    {
        match compiler.compile() {
            Ok(Some(dist)) => {
                let dist_notices = dist.notices;
                self.handle_notices(dist_notices);

                let write_notices = self.write_files(&dist.files);
                self.handle_notices(write_notices);
            }

            Ok(None) => {
                // Compiler is disabled, do nothing.
            }

            Err(err) => {
                self.handle_notices(GtNotice::error(format!("{err:?}")));
            }
        }
    }

    fn finalize(&mut self, dist_dir: &GtpDistDirPath) -> i32 {
        let errors_count = self.errors_count;
        if errors_count > 0 {
            self.backend.print_notice(GtNotice::warning(format!(
                "Project generated to `{dist_dir}` with {errors_count} errors"
            )));

            return 1;
        }

        self.backend.print_notice(GtNotice::success(format!(
            "Project generated to `{dist_dir}`"
        )));

        0
    }

    fn handle_notices<Notices: Into<Vec<GtNotice>>>(&mut self, notices: Notices) {
        let notices = notices.into();
        self.errors_count += notices
            .iter()
            .filter(|notice| matches!(notice.kind, GtNoticeKind::Error))
            .count();

        self.backend.print_notices(notices);
    }

    fn write_files(&self, files: &Vec<GtlDistFile>) -> Vec<GtNotice> {
        let mut notices = vec![];

        for file in files {
            let file_notices = self.write_file(file);
            notices.extend(file_notices);
        }

        notices
    }

    fn write_file(&self, file: &GtlDistFile) -> Vec<GtNotice> {
        let mut notices = vec![];
        let path = &file.path();
        let source_code = &file.source_code();

        let should_write = match file {
            GtlDistFile::Generated(_) => true,

            GtlDistFile::Error(error) => {
                // We only write the errored file if it doesn't exist in the file system, to avoid
                // overwriting existing files with errors.
                let file_exist_result = self.backend.file_exists(path.relative_path());

                match file_exist_result {
                    // Write to file system if it doesn't exist
                    Ok(false) => true,

                    Ok(true) | Err(_) => {
                        notices.push(GtNotice::error(format!(
                            "Failed to write `{path}` to file system as it was generated with errors: {message}",
                            message = error.message
                        )));

                        if let Err(err) = file_exist_result {
                            notices.push(GtNotice::error(format!(
                                "Failed to check if `{path}` exists in file system: {err}"
                            )));
                        }

                        false
                    }
                }
            }
        };

        if should_write {
            let write_result = self.backend.file_write(&path.relative_path(), source_code);
            if let Err(err) = write_result {
                notices.push(GtNotice::error(format!(
                    "Failed to write `{path}` to file system: {err}"
                )));
            }
        }

        notices
    }
}
