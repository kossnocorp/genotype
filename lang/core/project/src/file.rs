use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlProjectFile {
    pub path: GtCwdRelativePath,
    pub source: String,
}
