use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum TsProjectError {
    #[error("Failed to build module path from {0}")]
    #[diagnostic(code(GTTSP101))]
    BuildModulePath(String),
}
