use crate::prelude::internal::*;

// Dist path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtDistPath(GtCwdRelativePath);

impl GtRelativePath for GtDistPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtCwdRelativeDirPath<GtDistRelativePath> for GtDistPath {}

impl From<GtCwdRelativePath> for GtDistPath {
    fn from(path: GtCwdRelativePath) -> Self {
        Self(path)
    }
}

/// Path relative to the dist directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtDistRelativePath(RelativePathBuf);

impl GtRelativePath for GtDistRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path)
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

impl From<&str> for GtDistRelativePath {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}
