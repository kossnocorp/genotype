use std::path::PathBuf;

pub struct GTProjectOutputPath {
    path: PathBuf,
}

impl GTProjectOutputPath {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.path
    }
}
