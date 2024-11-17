use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum RSProjectError {
    #[error("Failed to build module path from {0}")]
    #[diagnostic(code(GTRSP101))]
    BuildModulePath(String),
}
