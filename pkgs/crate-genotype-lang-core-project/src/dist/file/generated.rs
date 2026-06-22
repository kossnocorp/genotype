use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlDistFileGenerated {
    pub path: GtpTargetFilePath,
    pub source_code: String,
}
