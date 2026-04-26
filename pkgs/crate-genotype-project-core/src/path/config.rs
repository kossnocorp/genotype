//! Project config-related paths. It is the first level of paths, relative to [GtpCwdPath].
//!
//! Often, the config dir would be equal to the cwd, but it can differ. It is the base path for
//! the config-related paths:
//!
//! - [super::root]
//!
//! Types:
//!
//! - [GtpConfigFilePath]: Path to the project config file. It encloses [GtpCwdRelativePath].
//! - [GtpConfigDirPath]: Path to the dir with the project config file. It encloses [GtpCwdRelativePath].
//! - [GtpConfigDirRelativePath]: Path relative to the project config dir. It encloses [RelativePathBuf].

use crate::prelude::internal::*;

// region: Config file path

gtp_cwd_relative_path_wrapper_newtype!(
    /// Config path relative to the working directory.
    pub struct GtpConfigFilePath(GtpCwdRelativePath);
);

impl GtpConfigFilePath {
    pub fn to_config_dir_path(&self) -> GtpConfigDirPath {
        self.0.to_parent().map_or_else(
            || GtpConfigDirPath::new(".".into()),
            GtpConfigDirPath::from_cwd_relative_path,
        )
    }
}

// endregion

// region: Config dir path

gtp_cwd_relative_dir_path_wrapper_newtype!(
    /// Config directory path relative to cwd.
    pub struct GtpConfigDirPath(GtpCwdRelativePath);
);

// endregion

// region: Config dir-relative path

gtp_relative_path_newtype!(
    /// Path relative to the project config directory.
    pub struct GtpConfigDirRelativePath;
    parent: GtpConfigDirPath;
);

// endregion
