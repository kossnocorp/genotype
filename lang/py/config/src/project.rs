use std::path::PathBuf;

use crate::lang::PYLangConfig;

pub struct PYProjectConfig {
    pub out: PathBuf,
    pub module: String,
    pub lang: PYLangConfig,
    pub package: Option<String>,
}

impl PYProjectConfig {
    pub fn package_path(&self, path: PathBuf) -> PathBuf {
        self.out.join(path)
    }

    pub fn source_path(&self, path: PathBuf) -> PathBuf {
        self.package_path(PathBuf::from(self.module.clone()).join(path))
    }
}
