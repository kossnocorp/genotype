use std::{
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

use genotype_parser::tree::GTPath;
use miette::Result;

use crate::error::GTProjectError;

pub mod resolve;
pub use resolve::*;

#[derive(Debug, Clone)]
pub struct GTPModulePath {
    // [TODO] Root must belong to the project not the module
    root: Arc<PathBuf>,
    // Canonical path to the module.
    path: PathBuf,
    id: GTPath,
}

impl GTPModulePath {
    pub fn as_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn as_id(&self) -> &GTPath {
        &self.id
    }

    pub fn as_name(&self) -> String {
        self.id.source_str().into()
    }

    pub fn as_path_str(&self) -> String {
        format!("{}.type", self.id.source_str())
    }

    pub fn resolve(&self, path: &GTPath) -> Result<Self> {
        let path = format!("{}.type", path.source_str());
        Self::try_new(
            Arc::clone(&self.root),
            &self.path.parent().unwrap().join(path),
        )
    }

    pub fn try_new(root: Arc<PathBuf>, path: &PathBuf) -> Result<Self> {
        let path = path.canonicalize().map_err(|_| {
            GTProjectError::CannotResolve(path.as_os_str().to_str().unwrap().to_owned())
        })?;
        let id = Self::id(&root, &path)?;
        Ok(Self { root, id, path })
    }

    pub fn id(root: &PathBuf, path: &PathBuf) -> Result<GTPath> {
        Ok(GTPath::parse(
            (0, 0).into(),
            path.as_path()
                .strip_prefix(root.as_path())
                .map_err(|_| GTProjectError::Unknown)?
                .with_extension("")
                .to_str()
                .unwrap(),
        )
        .unwrap())
    }
}

impl AsRef<Path> for GTPModulePath {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl PartialEq for GTPModulePath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for GTPModulePath {}

impl Hash for GTPModulePath {
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
        let module_path = GTPModulePath::try_new(root, &path).unwrap();
        assert_eq!(module_path.as_path(), &path.canonicalize().unwrap());
        assert_eq!(module_path.as_id().source_str(), "author")
    }
}
