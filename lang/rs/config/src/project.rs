use std::path::PathBuf;

use crate::lang::RSLangConfig;

#[derive(Debug, PartialEq, Clone)]
pub struct RSProjectConfig {
    pub out: PathBuf,
    pub lang: RSLangConfig,
    pub package: Option<String>,
}

impl RSProjectConfig {
    pub fn package_path(&self, path: PathBuf) -> PathBuf {
        self.out.join(path)
    }

    pub fn src_path(&self) -> PathBuf {
        self.package_path(PathBuf::from("src"))
    }

    pub fn source_path(&self, path: PathBuf) -> PathBuf {
        self.src_path().join(path)
    }
}
