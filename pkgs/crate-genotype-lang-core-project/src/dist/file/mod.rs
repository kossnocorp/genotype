use crate::prelude::internal::*;

mod error;
pub use error::*;

mod generated;
pub use generated::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtlDistFile {
    Error(GtlDistFileError),
    Generated(GtlDistFileGenerated),
}

impl GtlDistFile {
    pub fn path(&self) -> &GtpTargetFilePath {
        match self {
            GtlDistFile::Error(GtlDistFileError { path, .. })
            | GtlDistFile::Generated(GtlDistFileGenerated { path, .. }) => &path,
        }
    }

    pub fn source_code(&self) -> &String {
        match self {
            GtlDistFile::Error(GtlDistFileError { source_code, .. }) => &source_code,
            GtlDistFile::Generated(GtlDistFileGenerated { source_code, .. }) => &source_code,
        }
    }
}

impl<ProjectModule: GtlProjectModule> From<&GtlProjectModuleState<ProjectModule>> for GtlDistFile {
    fn from(module_state: &GtlProjectModuleState<ProjectModule>) -> Self {
        match module_state {
            GtlProjectModuleState::ConvertError(error) => {
                todo!();
            }

            GtlProjectModuleState::Converted(generated) => {
                todo!();
            }

            GtlProjectModuleState::RenderError(error) => {
                todo!();
            }

            GtlProjectModuleState::ResolveError(error) => {
                todo!();
            }

            GtlProjectModuleState::Resolved(resolved) => {
                todo!();
            }

            GtlProjectModuleState::Rendered(rendered) => {
                todo!();
            }
        }
    }
}

impl From<&GtlProjectFileExtra> for GtlDistFile {
    fn from(file: &GtlProjectFileExtra) -> Self {
        match file {
            GtlProjectFileExtra::Generated(generated) => {
                todo!();
            }

            GtlProjectFileExtra::Error(error) => {
                todo!();
            }
        }
    }
}

impl<ProjectModule: GtlProjectModule> From<&GtlProjectFile<ProjectModule>> for GtlDistFile {
    fn from(file: &GtlProjectFile<ProjectModule>) -> Self {
        match file {
            GtlProjectFile::Module(module_state) => module_state.into(),
            GtlProjectFile::Extra(extra_file) => extra_file.into(),
        }
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectFile<ProjectModule>> for GtlDistFile {
    fn from(file: GtlProjectFile<ProjectModule>) -> Self {
        (&file).into()
    }
}
