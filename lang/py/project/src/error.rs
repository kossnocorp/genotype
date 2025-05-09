use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum PyProjectError {
    #[error("Failed to build module path from {0}")]
    #[diagnostic(code(GTPYP101))]
    BuildModulePath(String),

    #[error("Failed to parse base manifest TOML")]
    #[diagnostic(code(GTRSP301))]
    ManifestBaseParse(#[source] toml_edit::TomlError),

    #[error("Failed to merge base manifest with config-specified: {")]
    #[diagnostic(code(GTRSP302))]
    ManifestMerge(#[source] toml_edit::TomlError),

    // [NOTE] GTPYP2XX is reserved for type checking, dependency resolution, etc.
    #[error("Failed to parse the [python.package]")]
    #[diagnostic(code(GTPYP303))]
    ParsePackage(#[from] toml_edit::TomlError),

    #[error("Failed to edit the pyproject.toml content: {0}")]
    #[diagnostic(code(GTPYP304))]
    EditPyproject(String),
}
