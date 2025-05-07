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
        Self(path)
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

#[cfg(feature = "parser")]
impl From<&GtSrcRelativePath> for GTModuleId {
    fn from(path: &GtSrcRelativePath) -> Self {
        path.relative_path().normalize().as_str().into()
    }
}

#[cfg(feature = "parser")]
impl From<GtSrcRelativePath> for GTModuleId {
    fn from(path: GtSrcRelativePath) -> Self {
        (&path).into()
    }
}

/// Entry pattern path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtEntryPath(GtCwdRelativePath);

impl GtRelativePath for GtEntryPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl From<GtCwdRelativePath> for GtEntryPath {
    fn from(path: GtCwdRelativePath) -> Self {
        Self(path)
    }
}
