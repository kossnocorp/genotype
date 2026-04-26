//! Src-related paths. It is the third level of paths, relative to [GtpRootDirPath].
//!
//! Src dir path is defined in the config and by default equals to "src". It is the base path for
//! the src-related paths:
//!
//! - [super::entry]
//! - [super::module]
//!
//! Types:
//!
//! - [GtpSrcDirPath]: Src dir path relative to [GtpCwdPath].
//! - [GtpRootDirRelativeSrcDirPath]: Src dir path relative to [GtpRootDirPath].
//! - [GtpSrcDirRelativePath]: Path relative to [GtpSrcDirPath].

use crate::prelude::internal::*;

// region: Src dir path

gtp_cwd_relative_dir_path_wrapper_newtype!(
    /// Src dir path relative to the cwd path.
    pub struct GtpSrcDirPath(GtpCwdRelativePath);
);

// endregion

// region: Root dir-relative src dir path

gtp_relative_path_wrapper_newtype!(
    /// Src dir path relative to the root dir.
    pub struct GtpRootDirRelativeSrcDirPath(GtpRootDirRelativePath);
    parent: GtpRootDirPath;
);

impl Default for GtpRootDirRelativeSrcDirPath {
    fn default() -> Self {
        Self::new("src".into())
    }
}

// endregion

// region: Src dir-relative path

gtp_relative_path_newtype!(
    /// Path relative to the src directory.
    pub struct GtpSrcDirRelativePath;
    parent: GtpSrcDirPath;
);

// endregion
