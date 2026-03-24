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

    fn join_path(&self, path: &RelativePathBuf) -> Self
    where
        Self: Sized,
    {
        Self::new(self.relative_path().join_normalized(path))
    }

    fn parent(&self) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(parent) = self.relative_path().parent() {
            Some(Self::new(parent.into()))
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
