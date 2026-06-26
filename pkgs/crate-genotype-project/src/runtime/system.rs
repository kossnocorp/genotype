use crate::prelude::internal::*;

use std::sync::{Arc, Mutex};

/// System project runtime. It combines parallel project loader with file system project source.
/// It is the default project runtime used by the CLI.
pub struct GtpRuntimeSystem {
    /// Base path for the project source to resolve relative file paths.
    base_path: GtpCwdRelativePath,
}

impl GtpRuntimeSystem {
    /// Creates a new system project runtime with the given base path.
    pub fn new(path: &GtpCwdRelativeOrAbsoluteStringPath) -> Result<Self> {
        let base_path = path
            .try_into()
            .wrap_err_with(|| format!("failed to normalize base path '{path}'"))?;
        Ok(Self { base_path })
    }

    /// Creates a new system project runtime and loads all modules for the project.
    pub fn new_and_load_all_modules(
        path: &GtpCwdRelativeOrAbsoluteStringPath,
        config_path: Option<&GtpCwdRelativeOrAbsoluteStringPath>,
    ) -> Result<GtProject> {
        let project_runtime =
            GtpRuntimeSystem::new(path).wrap_err("failed to create system project runtime")?;

        let config_path = config_path
            .map(|path| {
                path.try_into()
                    .wrap_err_with(|| format!("failed to normalize config path '{path}'"))
            })
            .transpose()?;

        let project = project_runtime
            .create_project(config_path.as_ref())
            .wrap_err("failed to create project")?;

        let project = project_runtime
            .load_all_modules(project)
            .wrap_err("failed to load all project modules")?;

        Ok(project)
    }
}

impl GtpLoaderParallel<GtpFileSourceSystemKind> for GtpRuntimeSystem {}

impl GtpFileAccessSystem for GtpRuntimeSystem {
    /// Returns the base project directory to resolve relative file paths.
    fn base_path(&self) -> &GtpCwdRelativePath {
        &self.base_path
    }
}

impl GtpFileSourceSystem for GtpRuntimeSystem {}

impl GtpFileSinkSystem for GtpRuntimeSystem {}

impl GtpDiagnosticSinkStdio for GtpRuntimeSystem {}

impl GtpRuntime for GtpRuntimeSystem {
    type LoaderKind = GtpLoaderParallelKind;

    type FileSourceKind = GtpFileSourceSystemKind;

    type FileSinkKind = GtpFileSinkSystemKind;

    type DiagnosticSinkKind = GtpDiagnosticSinkStdioKind;

    type ProjectRef = Arc<Mutex<GtProject>>;
}
