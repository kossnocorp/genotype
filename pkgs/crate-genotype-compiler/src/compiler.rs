use crate::prelude::internal::*;

pub struct GtCompiler<'project, 'backend> {
    /// Project to compile.
    project: &'project GtProject,

    /// Compiler backend to use for file system operations and notice printing.
    backend: &'backend dyn GtcBackend,

    /// Count of errors encountered during compilation.
    errors_count: usize,
}

impl GtCompiler<'_, '_> {
    pub fn new<'project, 'backend>(
        project: &'project GtProject,
        backend: &'backend dyn GtcBackend,
    ) -> GtCompiler<'project, 'backend> {
        GtCompiler {
            project,
            backend,
            errors_count: 0,
        }
    }

    pub fn compile(&mut self) -> i32 {
        let project = self.project;

        let project_notices = project.as_notices();
        self.handle_notices(project_notices);

        self.compile_project(&TsCompiler::new(project));
        self.compile_project(&PyCompiler::new(project));
        self.compile_project(&RsCompiler::new(project));

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
                let file_exist_result = self.backend.file_exists(&path.to_path_buf());
                if !file_exist_result {
                    notices.push(GtNotice::error(format!(
                        "Failed to write `{path}` to file system as it was generated with errors: {message}",
                        message = error.message
                    )));
                }
                !file_exist_result
            }
        };

        if should_write {
            let write_result = Self::write_file_source_code(path, source_code);
            if let Err(err) = write_result {
                notices.push(GtNotice::error(format!(
                    "Failed to write `{path}` to file system: {err}"
                )));
            }
        }

        notices
    }

    fn write_file_source_code(path: &GtpTargetFilePath, source_code: &String) -> Result<()> {
        let parent_dir_path = path
            .to_parent()
            .ok_or_else(|| miette!("Failed to get parent directory for `{path}`"))?;

        fs::create_dir_all(parent_dir_path.to_path_buf())
            .map_err(|err| miette!(err))
            .wrap_err_with(|| format!("Failed to create directory `{parent_dir_path}`"))?;

        fs::write(path.to_path_buf(), source_code)
            .map_err(|err| miette!(err))
            .wrap_err_with(|| format!("Failed to write file `{path}`"))?;

        Ok(())
    }
}
