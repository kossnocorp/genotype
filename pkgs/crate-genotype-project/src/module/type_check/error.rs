use crate::prelude::internal::*;

#[derive(Error, Debug, Diagnostic, PartialEq, Clone, Serialize)]
pub enum GtpModuleTypeCheckError {
    #[error("Invalid record key `{identifier}`: {reason}")]
    #[diagnostic(code(GTP301))]
    InvalidRecordKey {
        #[label("Record key referenced here")]
        span: GtSpan,
        identifier: String,
        reason: &'static str,
    },
}

impl GtpModuleTypeCheckError {
    pub fn span(&self) -> GtSpan {
        match self {
            GtpModuleTypeCheckError::InvalidRecordKey { span, .. } => *span,
        }
    }
}
