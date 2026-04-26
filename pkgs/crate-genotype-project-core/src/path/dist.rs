//! Dist-related paths. It is the third level of paths, relative to [GtpRootDirRelativePath].
//!
//! Dist dir path is defined in the config and by default equals to "dist". It is the base path for
//! the dist-related paths:
//!
//! - [super::pkg]
//!
//! Types:
//!
//! - [GtpDistDirPath]: Dist dir path relative to [GtpCwdPath].
//! - [GtpRootDirRelativeDistDirPath]: Dist dir path relative to [GtpRootDirPath].
//! - [GtpDistDirRelativePath]: Path relative to [GtpDistDirPath].

use crate::prelude::internal::*;

// region: Dist dir path

gtp_cwd_relative_dir_path_wrapper_newtype!(
    /// Dist dir path relative to cwd path.
    pub struct GtpDistDirPath(GtpCwdRelativePath);
);

// endregion

// region: Root dir-relative dist dir path

gtp_relative_path_wrapper_newtype!(
    /// Dist dir path relative to the root dir.
    pub struct GtpRootDirRelativeDistDirPath(GtpRootDirRelativePath);
    parent: GtpRootDirPath;
);

impl Default for GtpRootDirRelativeDistDirPath {
    fn default() -> Self {
        Self::new("dist".into())
    }
}

// endregion

// region: Dist dir-relative path

gtp_relative_path_newtype!(
    /// Path relative to the dist directory.
    pub struct GtpDistDirRelativePath;
    parent: GtpDistDirPath;
);

// endregion
