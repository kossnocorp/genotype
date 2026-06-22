use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Error, Diagnostic, Serialize)]
pub enum GtlConfigError {
    #[error("Failed to resolve path `{source_path}`: {message}")]
    ResolveTargetModulePath {
        source_path: GtpModulePath,
        message: String,
    },
}
