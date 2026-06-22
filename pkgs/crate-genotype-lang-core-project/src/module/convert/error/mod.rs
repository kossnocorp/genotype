use crate::prelude::internal::*;

mod state;
pub use state::*;

mod source_state;
pub use source_state::*;

#[derive(Debug, Clone, PartialEq, Error, Diagnostic, Serialize)]
pub enum GtlProjectModuleConvertError {
    #[error(
        "Failed to convert `{target_path}` from `{path}` because it is in `{source_state:?}` state", path = source.path()
    )]
    SourceState {
        r#source: GtpModuleSource, // SEE: !CMNT_THISERROR_SOURCE
        #[source]
        source_state: GtlProjectModuleConvertErrorSourceState,
        target_path: GtpTargetFilePath,
    },

    #[error("Failed to resolve module path from `{source_path}`")]
    ResolvePath {
        source_path: GtpModulePath,
        #[source]
        error: GtlConfigError,
    },

    #[error("Failed to convert `{target_path}` from `{source_path}`")]
    ConvertError {
        source_path: GtpModulePath,
        target_path: GtpTargetFilePath,
        #[source]
        #[diagnostic_source]
        error: Box<dyn GtlError>,
    },
}

impl GtlProjectModuleStateInner for GtlProjectModuleConvertError {
    fn source_path(&self) -> &GtpModulePath {
        match self {
            Self::SourceState { source, .. } => source.path(),
            Self::ResolvePath { source_path, .. } | Self::ConvertError { source_path, .. } => {
                source_path
            }
        }
    }

    fn target_path(&self) -> Option<&GtpTargetFilePath> {
        match self {
            Self::SourceState { target_path, .. } | Self::ConvertError { target_path, .. } => {
                Some(target_path)
            }
            Self::ResolvePath { .. } => None,
        }
    }
}

impl<ProjectModule: GtlProjectModule> Into<GtlProjectModuleState<ProjectModule>>
    for GtlProjectModuleConvertError
{
    fn into(self) -> GtlProjectModuleState<ProjectModule> {
        GtlProjectModuleState::ConvertError(GtlProjectModuleConvertErrorState { error: self })
    }
}
