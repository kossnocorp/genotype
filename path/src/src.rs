use crate::prelude::internal::*;

// Src path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtSrcPath(GtCwdRelativePath);

impl GtRelativePath for GtSrcPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtCwdRelativeDirPath<GtSrcRelativePath> for GtSrcPath {}

impl From<GtCwdRelativePath> for GtSrcPath {
    fn from(path: GtCwdRelativePath) -> Self {
        Self(path)
    }
}

/// Path relative to the src directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtSrcRelativePath(RelativePathBuf);

impl GtRelativePath for GtSrcRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

impl From<&str> for GtSrcRelativePath {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}
