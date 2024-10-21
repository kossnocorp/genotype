use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub struct GTLangProjectSource {
    pub path: PathBuf,
    pub code: String,
}
