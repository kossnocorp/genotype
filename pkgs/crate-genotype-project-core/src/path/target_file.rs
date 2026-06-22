//! Target file-related paths. It is the fifth level of paths, relative to [GtpPkgDirPath].
//!
//! Target file path is the path to Genotype target file.
//!
//! Types:
//!
//! - [GtpTargetFilePath]: Target file path relative to [GtpCwdPath].
//! - [GtpPkgDirRelativeTargetFilePath]: Target file path relative to [GtpPkgDirPath].

use crate::prelude::internal::*;

// region: Cwd-relative target file path

gtp_cwd_relative_path_wrapper_newtype!(
    /// Target file path relative to cwd.
    pub struct GtpTargetFilePath(GtpCwdRelativePath);
);

// endregion

// region: Package dir-relative target file path

gtp_relative_path_wrapper_newtype!(
    /// Target file path relative to the src directory.
    pub struct GtpPkgDirRelativeTargetFilePath(GtpPkgDirRelativePath);
    parent: GtpPkgDirPath;
);

// endregion
