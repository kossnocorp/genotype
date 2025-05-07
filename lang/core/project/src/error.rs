use crate::prelude::internal::*;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum GtlProjectError {
    #[error("Failed to convert manifest TOML")]
    #[diagnostic(code(GTRSP301))]
    ManifestTomlConvert(#[from] toml_edit::ser::Error),

    #[error("Failed to adjust {0}: {1}")]
    #[diagnostic(code(GTRSP302))]
    ManifestAdjust(&'static str, String),
}
