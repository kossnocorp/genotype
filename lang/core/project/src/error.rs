use crate::prelude::internal::*;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum GtlProjectError {
    #[error("Failed to merge dependencies in the manifest {0}")]
    #[diagnostic(code(GTRSP301))]
    ManifestDepsMerge(&'static str),

    #[error("Failed to access dependencies in the manifest {0}")]
    #[diagnostic(code(GTRSP302))]
    ManifestDepsAccess(&'static str),

    #[error("Failed to convert manifest TOML")]
    #[diagnostic(code(GTRSP303))]
    ManifestTomlConvert(#[from] toml_edit::ser::Error),

    #[error("Failed to adjust {0}: {1}")]
    #[diagnostic(code(GTRSP304))]
    ManifestAdjust(&'static str, String),

    #[error("Failed to format {0}")]
    #[diagnostic(code(GTRSP305))]
    ManifestFormat(&'static str),
}
