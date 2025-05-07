use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub struct GtlProjectFile {
    pub path: PathBuf,
    pub source: String,
}
