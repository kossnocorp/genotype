use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct GTProjectOutPath {
    path: PathBuf,
}

impl GTProjectOutPath {
    pub fn new(path: &PathBuf) -> Self {
        Self { path: path.clone() }
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.path
    }
}

impl From<&str> for GTProjectOutPath {
    fn from(path: &str) -> Self {
        Self::new(&PathBuf::from(path))
    }
}

impl From<PathBuf> for GTProjectOutPath {
    fn from(path: PathBuf) -> Self {
        Self::new(&path)
    }
}

impl PartialEq for GTProjectOutPath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for GTProjectOutPath {}

impl Hash for GTProjectOutPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
