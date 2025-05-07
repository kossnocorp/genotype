use crate::prelude::internal::*;

pub trait GtRelativePath {
    fn new(path: RelativePathBuf) -> Self;

    fn relative_path(&self) -> &RelativePathBuf;

    fn as_str(&self) -> &str {
        self.relative_path().as_str()
    }

    #[cfg(feature = "parser")]
    fn join_tree(&self, path: &GTPath) -> Self
    where
        Self: Sized,
    {
        Self::new(self.relative_path().join(path.source_str()))
    }
}

impl GtRelativePath for RelativePathBuf {
    fn new(path: RelativePathBuf) -> Self {
        path
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self
    }
}
