use std::{
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

use genotype_parser::tree::GTPath;

#[derive(Debug, Clone)]
pub struct GTProjectModulePath {
    root: Arc<PathBuf>,
    path: PathBuf,
    id: GTPath,
}

impl GTProjectModulePath {
    pub fn as_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn as_id(&self) -> &GTPath {
        &self.id
    }

    pub fn resolve(&self, path: &GTPath) -> Result<Self, Box<dyn std::error::Error>> {
        let path = format!("{}.type", path.as_str());
        Self::try_new(
            Arc::clone(&self.root),
            &self.path.parent().unwrap().join(path),
        )
    }

    pub fn try_new(root: Arc<PathBuf>, path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.canonicalize()?;
        let id = Self::id(&root, &path)?;
        Ok(Self { root, id, path })
    }

    pub fn id(root: &PathBuf, path: &PathBuf) -> Result<GTPath, Box<dyn std::error::Error>> {
        Ok(path
            .as_path()
            .strip_prefix(root.as_path())?
            .with_extension("")
            .to_str()
            .unwrap()
            .into())
    }
}

impl AsRef<Path> for GTProjectModulePath {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl PartialEq for GTProjectModulePath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for GTProjectModulePath {}

impl Hash for GTProjectModulePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_try_new() {
        let root = Arc::new(PathBuf::from("./examples/basic").canonicalize().unwrap());
        let path = root.join("author.type");
        let module_path = GTProjectModulePath::try_new(root, &path).unwrap();
        assert_eq!(module_path.as_path(), &path.canonicalize().unwrap());
        assert_eq!(module_path.as_id().as_str(), "author")
    }
}
