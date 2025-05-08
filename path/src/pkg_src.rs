use crate::prelude::internal::*;

// Package src path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtPkgSrcPath(GtCwdRelativePath);

impl GtRelativePath for GtPkgSrcPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtCwdRelativeDirPath<GtPkgSrcRelativePath> for GtPkgSrcPath {}

impl From<GtCwdRelativePath> for GtPkgSrcPath {
    fn from(path: GtCwdRelativePath) -> Self {
        Self(path)
    }
}

/// Path relative to the target package src directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtPkgSrcRelativePath(RelativePathBuf);

impl GtRelativePath for GtPkgSrcRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path)
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

impl From<&str> for GtPkgSrcRelativePath {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}
