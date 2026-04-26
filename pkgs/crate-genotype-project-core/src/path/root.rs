//! Root-related paths. It is the second level of paths, relative to [GtpConfigDirPath].
//!
//! Root path is defined in the config and by default equals to the config dir. It is the base path
//! for the root-related paths:
//!
//! - [super::src]
//! - [super::dist]
//!
//! Types:
//!
//! - [GtpRootDirPath]: Root dir path relative to [GtpCwdPath].
//! - [GtpConfigDirRelativeRootDirPath]: Root dir path relative to [GtpConfigDirPath].
//! - [GtpRootDirRelativePath]: Generic path relative to [GtpRootDirPath].

use crate::prelude::internal::*;

// region: Root dir path

gtp_cwd_relative_dir_path_wrapper_newtype!(
    /// Root path relative to cwd.
    pub struct GtpRootDirPath(GtpCwdRelativePath);
);

// endregion

// region: Config-dir relative root dir path

gtp_relative_path_wrapper_newtype!(
    /// Root path relative to config dir.
    pub struct GtpConfigDirRelativeRootDirPath(GtpConfigDirRelativePath);
    parent: GtpConfigDirPath;
);

impl Default for GtpConfigDirRelativeRootDirPath {
    fn default() -> Self {
        Self(".".into())
    }
}

// endregion

// region: Root dir-relative path

gtp_relative_path_newtype!(
    /// Path relative to root dir path.
    pub struct GtpRootDirRelativePath;
    parent: GtpRootDirPath;
);

// endregion
