//! Package src-related paths. It is the fifth level of paths, relative to [GtpPkgDirPath].
//!
//! Package src path is a path to the target package src directory, e.g., `"<root>/dist/rs/src"`.
//!
//! Types:
//!
//! - [GtpPkgSrcDirPath]: Package src dir path relative to [GtpCwdPath].
//! - [GtpPkgDirRelativePkgSrcDirPath]: Package src dir path relative to [GtpPkgDirPath].
//! - [GtpPkgSrcDirRelativePath]: Path relative to [GtpPkgSrcDirPath].

use crate::prelude::internal::*;

// region: Types

// region: Package src dir path

gtp_cwd_relative_dir_path_wrapper_newtype!(
    /// Package src dir path relative to cwd.
    pub struct GtpPkgSrcDirPath(GtpCwdRelativePath);
);

// endregion

// region: Package dir-relative package src dir path

gtp_relative_path_wrapper_newtype!(
    /// Package src dir path relative to package dir.
    pub struct GtpPkgDirRelativePkgSrcDirPath(GtpPkgDirRelativePath);
    parent: GtpPkgDirPath;
);

// endregion

// region: Package src dir-relative path

gtp_relative_path_newtype!(
    /// Path relative to package src dir.
    pub struct GtpPkgSrcDirRelativePath;
    parent: GtpPkgSrcDirPath;
);

// endregion

// endregion
