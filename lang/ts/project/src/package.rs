use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct TSPackage {
    pub types: String,
    // [TODO] Merge with package?
    // pub files: Vec<String>,
    #[serde(flatten)]
    pub package: Option<toml::Value>,
}
