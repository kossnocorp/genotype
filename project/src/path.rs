use std::{
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct GTProjectPath {
    path: PathBuf,
    parent: PathBuf,
}

impl GTProjectPath {
    pub fn try_new(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.canonicalize()?;
        let parent = path.parent().unwrap().to_path_buf();
        Ok(Self { path, parent })
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn resolve(&self, path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = format!("{}.type", path);
        Self::try_new(&self.parent.join(path))
    }
}

impl TryFrom<&str> for GTProjectPath {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::try_new(&PathBuf::from(path))
    }
}

impl TryFrom<PathBuf> for GTProjectPath {
    type Error = Box<dyn std::error::Error>;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::try_new(&path)
    }
}

impl AsRef<Path> for GTProjectPath {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl PartialEq for GTProjectPath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for GTProjectPath {}

impl Hash for GTProjectPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}
