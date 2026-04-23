use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlProjectFile {
    pub path: GtpCwdRelativePath,
    pub source: String,
}
