use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub struct TSProjectConfig {
    pub out: PathBuf,
    pub src: PathBuf,
    pub package: Option<toml::Value>,
    /// Manually mapped dependencies.
    pub dependencies: Option<HashMap<String, String>>,
}

impl TSProjectConfig {
    pub fn package_path(&self, path: PathBuf) -> PathBuf {
        self.out.join(path)
    }

    pub fn source_path(&self, path: PathBuf) -> PathBuf {
        self.package_path(PathBuf::from(self.src.clone()).join(path))
    }
}
