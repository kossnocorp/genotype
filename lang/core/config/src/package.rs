use crate::prelude::internal::*;

/// Target language package setting trait. Its primary purpose is to provide a default path for config
/// during initialization and parse.
pub trait GtlConfigPkgPathSetting: Default {
    const DEFAULT: &'static str;

    fn path<'a>(&'a self) -> &'a GtDistRelativePath;

    fn default_relative_path() -> GtDistRelativePath {
        GtDistRelativePath::new(Self::DEFAULT.into())
    }
}
