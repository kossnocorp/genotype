use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GtlProjectFile {
    pub path: GtCwdRelativePath,
    pub source: String,
}
