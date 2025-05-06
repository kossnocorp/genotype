use genotype_parser::GTSpan;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum GTProjectError {
    #[error("`{0}` is not found")]
    #[diagnostic(code(GTP101))]
    Canonicalize(String),

    #[error("no entries found for pattern \"{0}\"")]
    #[diagnostic(code(GTP102))]
    NoEntries(String),

    #[error("failed to read `{0}`")]
    #[diagnostic(code(GTP103))]
    NotFound(String),

    #[error("failed to resolve `{0}`")]
    #[diagnostic(code(GTP104))]
    CannotResolve(String),

    #[error("YO")]
    #[diagnostic(code(YO))]
    YO,

    #[error("undefined type `{identifier}`")]
    #[diagnostic(code(GTP201))]
    UndefinedType {
        #[label("referenced here")]
        span: GTSpan,
        identifier: String,
    },

    #[error("unknown error")]
    #[diagnostic(code(GTP999))]
    Unknown,
}
