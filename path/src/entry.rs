use crate::prelude::internal::*;

/// Entry pattern path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtEntryPath(RelativePathBuf);

impl GtEntryPath {
    pub fn with_parent(&self, path: &RelativePathBuf) -> GtEntryPath {
        GtEntryPath::new(path.join(self.relative_path()))
    }
}

impl GtRelativePath for GtEntryPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path)
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}
