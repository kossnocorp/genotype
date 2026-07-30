use crate::prelude::internal::*;

#[derive(Error, Debug, Diagnostic, PartialEq, Clone, Serialize)]
pub enum GtpError {
    #[error("Undefined type `{identifier}`: {reason}")]
    #[diagnostic(code(GTP201))]
    UndefinedType {
        #[label("referenced here")]
        span: GtSpan,
        identifier: String,
        reason: &'static str,
    },

    #[error("Unknown error")]
    #[diagnostic(code(GTP999))]
    Unknown,
}
