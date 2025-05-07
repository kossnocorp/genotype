use crate::prelude::internal::*;

pub trait GtCwdRelativeDirPath<ChildPath: GtRelativePath>: GtRelativePath {
    fn join(&self, path: &ChildPath) -> GtCwdRelativePath {
        GtCwdRelativePath::new(self.relative_path().join(path.relative_path()))
    }
}

/// Path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtCwdRelativePath(RelativePathBuf);

impl GtRelativePath for GtCwdRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path)
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

impl From<&str> for GtCwdRelativePath {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}
