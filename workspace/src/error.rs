use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum GTWError {
    #[error("Failed to lock the files map")]
    #[diagnostic(code(GTW001))]
    FilesLock,

    #[error("`{0}` not found")]
    #[diagnostic(
        code(GTW101),
        help("Make sure the path is absolute or can be resolved from the current directory")
    )]
    Canonicalize(String),
}
