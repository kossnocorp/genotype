use std::path::PathBuf;

use crate::lang::RSLangConfig;

pub struct RSProjectConfig {
    pub out: PathBuf,
    pub lang: RSLangConfig,
    pub package: Option<String>,
}

impl RSProjectConfig {
    pub fn package_path(&self, path: PathBuf) -> PathBuf {
        self.out.join(path)
    }

    // pub fn module_root_path(&self) -> PathBuf {
    //     self.package_path(PathBuf::from(self.module.clone()))
    // }

    pub fn source_path(&self, path: PathBuf) -> PathBuf {
        self.package_path(PathBuf::from("src").join(path))
    }
}
