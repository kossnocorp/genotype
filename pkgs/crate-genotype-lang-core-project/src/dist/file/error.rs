use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlDistFileError {
    pub path: GtpTargetFilePath,
    pub message: String,
    pub source_code: String,
}
