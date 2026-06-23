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
        let path = module_state.target_path().cloned().unwrap_or_else(|| {
            GtpTargetFilePath::from(module_state.source_path().cwd_relative_path().clone())
        });

        match module_state {
            GtlProjectModuleState::ConvertError(_) => GtlDistFile::Error(GtlDistFileError {
                message: format_module_state_error_message(module_state, &path),
                path,
                source_code: String::new(),
            }),

            GtlProjectModuleState::Converted(_) => GtlDistFile::Error(GtlDistFileError {
                path,
                message: format!("Module is in an invalid state '{}'", module_state.name()),
                source_code: String::new(),
            }),

            GtlProjectModuleState::RenderError(_) => GtlDistFile::Error(GtlDistFileError {
                message: format_module_state_error_message(module_state, &path),
                path,
                source_code: String::new(),
            }),

            GtlProjectModuleState::ResolveError(_) => GtlDistFile::Error(GtlDistFileError {
                message: format_module_state_error_message(module_state, &path),
                path,
                source_code: String::new(),
            }),

            GtlProjectModuleState::Resolved(_) => GtlDistFile::Error(GtlDistFileError {
                path,
                message: format!("Module is in an invalid state '{}'", module_state.name()),
                source_code: String::new(),
            }),

            GtlProjectModuleState::Rendered(rendered) => {
                GtlDistFile::Generated(GtlDistFileGenerated {
                    path,
                    source_code: rendered.source_code.clone(),
                })
            }
        }
    }
}

impl From<&GtlProjectFileExtra> for GtlDistFile {
    fn from(file: &GtlProjectFileExtra) -> Self {
        match file {
            GtlProjectFileExtra::Generated(generated) => {
                GtlDistFile::Generated(GtlDistFileGenerated {
                    path: generated.path.clone(),
                    source_code: generated.source_code.clone(),
                })
            }

            GtlProjectFileExtra::Error(error) => match error {
                GtlProjectFileExtraError::Generate { target_path, .. } => {
                    GtlDistFile::Error(GtlDistFileError {
                        path: target_path.clone(),
                        message: format!("{}", error),
                        source_code: String::new(),
                    })
                }
            },
        }
    }
}

fn format_module_state_error_message<ProjectModule: GtlProjectModule>(
    module_state: &GtlProjectModuleState<ProjectModule>,
    target_path: &GtpTargetFilePath,
) -> String {
    let action = module_state.action();
    let source_path = module_state.source_path();
    format!("Failed to {action} `{target_path}` from `{source_path}`")
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
