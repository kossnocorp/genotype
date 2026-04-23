use crate::prelude::internal::*;

#[derive(Error, Debug, Diagnostic, Clone)]
pub enum GtpError {
    #[error("no entries found for pattern \"{0}\"")]
    #[diagnostic(code(GTP103))]
    #[deprecated(note = "use `miette!` instead")]
    NoEntries(String),

    #[error("failed to read `{0}`")]
    #[diagnostic(code(GTP104))]
    #[deprecated(note = "use `miette!` instead")]
    NotFound(String),

    #[error("undefined type `{identifier}`")]
    #[diagnostic(code(GTP301))]
    UndefinedType {
        #[label("referenced here")]
        span: GtSpan,
        identifier: String,
    },

    #[error("unknown error")]
    #[diagnostic(code(GTP999))]
    Unknown,
}
