use crate::prelude::internal::*;

/// Out target package setting trait. Its primary purpose is to provide a default path for config
/// during initialization and parse.
pub trait GtlConfigPkgPathSetting: Default {
    const DEFAULT: &'static str;

    fn relative_path<'a>(&'a self) -> &'a RelativePathBuf;

    fn to_path(&self) -> GtDistRelativePath {
        GtDistRelativePath::new(self.relative_path().clone())
    }

    fn default_relative_path() -> RelativePathBuf {
        Self::DEFAULT.into()
    }
}
