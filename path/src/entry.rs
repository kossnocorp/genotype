use crate::prelude::internal::*;

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
