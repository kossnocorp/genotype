use std::path::PathBuf;

use crate::lang::PYLangConfig;

pub struct PYProjectConfig {
    pub out: PathBuf,
    pub module: String,
    pub lang: PYLangConfig,
}

impl PYProjectConfig {
    pub fn source_path(&self, path: PathBuf) -> PathBuf {
        self.out.join(self.module.clone()).join(path)
    }
}
