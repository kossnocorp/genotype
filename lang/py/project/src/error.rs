use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum PyProjectError {
    #[error("Failed to build module path from {0}")]
    #[diagnostic(code(GTPYP101))]
    BuildModulePath(String),

    // [NOTE] GTPYP2XX is reserved for type checking, dependency resolution, etc.
    #[error("Failed to parse the [python.package]")]
    #[diagnostic(code(GTPYP301))]
    ParsePackage(#[from] toml_edit::TomlError),

    #[error("Failed to edit the pyproject.toml content: {0}")]
    #[diagnostic(code(GTPYP302))]
    EditPyproject(String),
}
