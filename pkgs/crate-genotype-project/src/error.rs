use crate::prelude::internal::*;

#[derive(Error, Debug, Diagnostic, PartialEq, Clone, Serialize)]
pub enum GtpError {
    #[error("undefined type `{identifier}`: {reason}")]
    #[diagnostic(code(GTP301))]
    UndefinedType {
        #[label("referenced here")]
        span: GtSpan,
        identifier: String,
        reason: &'static str,
    },

    #[error("unknown error")]
    #[diagnostic(code(GTP999))]
    Unknown,
}
