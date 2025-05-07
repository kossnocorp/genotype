use crate::prelude::internal::*;

pub trait GtRelativePath {
    fn new(path: RelativePathBuf) -> Self;

    fn relative_path(&self) -> &RelativePathBuf;

    fn as_str(&self) -> &str {
        self.relative_path().as_str()
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
