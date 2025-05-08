use crate::prelude::internal::*;
use std::iter::Successors;

pub trait GtRelativePath {
    fn new(path: RelativePathBuf) -> Self;

    fn relative_path(&self) -> &RelativePathBuf;

    fn as_str(&self) -> &str {
        self.relative_path().as_str()
    }

    fn with_extension<Str: AsRef<str>>(&self, ext: Str) -> Self
    where
        Self: Sized,
    {
        Self::new(self.relative_path().with_extension(ext))
    }

    fn strip_extension(&self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.relative_path().strip_extension())
    }

    fn join_path(&self, path: &RelativePathBuf) -> Self
    where
        Self: Sized,
    {
        Self::new(self.relative_path().join(path))
    }

    #[cfg(feature = "parser")]
    fn join_tree(&self, path: &GTPath) -> Self
    where
        Self: Sized,
    {
        Self::new(self.relative_path().join(path.source_str()))
    }

    fn parent(&self) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(parent) = self.relative_path().parent() {
            Some(Self::new(parent))
        } else {
            None
        }
    }

    fn parents(&self) -> Successors<Self, fn(&Self) -> Option<Self>>
    where
        Self: Sized + Clone,
    {
        std::iter::successors(self.parent(), Self::parent)
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
