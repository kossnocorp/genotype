//! Package-related paths. It is the fourth level of paths, relative to [GtpDistDirPath].
//!
//! Package path is a path to the target package directory, e.g., `"<root>/dist/rs"`. It is
//! the base path for the package src-related paths:
//!
//! - [super::pkg_src]
//!
//! Types:
//!
//! - [GtpPkgDirPath]: Package dir path relative to [GtpCwdPath].
//! - [GtpDistDirRelativePkgDirPath]: Package dir path relative to [GtpDistDirPath].
//! - [GtpPkgDirRelativePath]: Path relative to [GtpPkgDirPath].

use crate::prelude::internal::*;

// region: Package dir path

gtp_cwd_relative_dir_path_wrapper_newtype!(
    /// Package dir path relative to cwd.
    pub struct GtpPkgDirPath(GtpCwdRelativePath);
);

// impl GtpPkgDirPath {
//     pub fn join_as_pkg_dir_relative_path(&self, path: Gtp) -> GtpPkgDirRelativePath {
//         GtpPkgDirRelativePath::
//     }
// }

// endregion

// region: Dist dir-relative package dir path

gtp_relative_path_wrapper_newtype!(
    /// Package dir path relative to dist dir.
    pub struct GtpDistDirRelativePkgDirPath(GtpDistDirRelativePath);
    parent: GtpDistDirPath;
);

impl GtpDistDirRelativePkgDirPath {
    pub fn join_as_dist_dir_relative_path(
        &self,
        path: &GtpPkgDirRelativePath,
    ) -> GtpDistDirRelativePath {
        GtpDistDirRelativePath::new(self.relative_path().join(path.relative_path()))
    }
}

// endregion

// region: Package dir-relative path

gtp_relative_path_newtype!(
    /// Path relative to package dir.
    pub struct GtpPkgDirRelativePath;
    parent: GtpPkgDirPath;
);

// endregion
