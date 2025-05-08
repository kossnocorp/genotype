use crate::prelude::internal::*;

// Package path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtPkgPath(GtCwdRelativePath);

impl GtRelativePath for GtPkgPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtCwdRelativeDirPath<GtPkgRelativePath> for GtPkgPath {}

impl From<GtCwdRelativePath> for GtPkgPath {
    fn from(path: GtCwdRelativePath) -> Self {
        Self(path)
    }
}

/// Path relative to the target package directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtPkgRelativePath(RelativePathBuf);

impl GtRelativePath for GtPkgRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

impl From<&str> for GtPkgRelativePath {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}
