use std::path::PathBuf;

pub struct RSProjectConfig {
    pub out: PathBuf,
    pub package: Option<toml::Value>,
}

pub const RS_SRC: &str = "src";

impl RSProjectConfig {
    pub fn package_path(&self, path: PathBuf) -> PathBuf {
        self.out.join(path)
    }

    pub fn source_path(&self, path: PathBuf) -> PathBuf {
        self.package_path(PathBuf::from(RS_SRC).join(path))
    }
}
