use crate::prelude::internal::*;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum GtlProjectError {
    #[error("Failed to parse {0} TOML")]
    #[diagnostic(code(GTRSP301))]
    ManifestTomlParse(&'static str, #[source] toml_edit::TomlError),

    #[error("Failed to merge dependencies in the manifest {0}")]
    #[diagnostic(code(GTRSP302))]
    ManifestDepsMerge(&'static str),

    #[error("Failed to access dependencies in the manifest {0}")]
    #[diagnostic(code(GTRSP303))]
    ManifestDepsAccess(&'static str),

    #[error("Failed to convert manifest TOML")]
    #[diagnostic(code(GTRSP304))]
    ManifestTomlConvert(#[from] toml_edit::ser::Error),

    #[error("Failed to adjust {0}: {1}")]
    #[diagnostic(code(GTRSP305))]
    ManifestAdjust(&'static str, String),

    #[error("Failed to format {0}")]
    #[diagnostic(code(GTRSP306))]
    ManifestFormat(&'static str),
}
