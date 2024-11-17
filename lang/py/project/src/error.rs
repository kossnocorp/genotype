use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum PYProjectError {
    #[error("Failed to build module path from {0}")]
    #[diagnostic(code(GTPYP101))]
    BuildModulePath(String),
}
