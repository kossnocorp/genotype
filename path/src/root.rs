use crate::prelude::internal::*;

// Root path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtRootPath(GtCwdRelativePath);

impl GtRelativePath for GtRootPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtCwdRelativeDirPath<GtRootRelativePath> for GtRootPath {}

impl From<GtCwdRelativePath> for GtRootPath {
    fn from(path: GtCwdRelativePath) -> Self {
        Self(path)
    }
}

/// Path relative to the project root directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtRootRelativePath(RelativePathBuf);

impl GtRelativePath for GtRootRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path)
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

impl From<&str> for GtRootRelativePath {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}
