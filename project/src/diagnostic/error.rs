use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GTProjectError {
    #[error("`{0}` not found")]
    #[diagnostic(code(GTP101))]
    Canonicalize(String),

    #[error("no entries found for pattern \"{0}\"")]
    #[diagnostic(code(GTP102))]
    NoEntries(String),

    #[error("unknown error")]
    #[diagnostic(code(GTP999))]
    Unknown,
}
