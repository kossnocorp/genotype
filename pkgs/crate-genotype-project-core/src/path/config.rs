//! Project config-related paths. It is the first level of paths, relative to [GtpCwdPath].
//!
//! Often, the config dir would be equal to the cwd, but it can differ. It is the base path for
//! the config-related paths:
//!
//! - [super::root]
//! Traits:
//!
//! - [GtpConfigDirRelativePathWrapper]: Trait for paths that wraps [GtpConfigDirRelativePath] and can be converted to it.
//!
//! Types:
//!
//! - [GtpConfigFilePath]: Path to the project config file. It encloses [GtpCwdRelativePath].
//! - [GtpConfigDirPath]: Path to the dir with the project config file. It encloses [GtpCwdRelativePath].
//! - [GtpConfigDirRelativePath]: Path relative to the project config dir. It encloses [RelativePathBuf].

use crate::prelude::internal::*;

// region: Traits

// region: Config dir-relative path wrapper path trait

pub trait GtpConfigDirRelativePathWrapper: GtpRelativePath {}

// endregion

// endregion

// region: Types

// region: Config file path

/// Config path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpConfigFilePath(GtpCwdRelativePath);

impl GtpRelativePath for GtpConfigFilePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl From<GtpCwdRelativePath> for GtpConfigFilePath {
    fn from(path: GtpCwdRelativePath) -> Self {
        Self(path)
    }
}

// endregion

// region: Config dir path

/// Config directory path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpConfigDirPath(GtpCwdRelativePath);

impl GtpRelativePath for GtpConfigDirPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtpCwdRelativeDirPathWrapper<GtpConfigDirRelativePath> for GtpConfigDirPath {}

impl From<GtpCwdRelativePath> for GtpConfigDirPath {
    fn from(path: GtpCwdRelativePath) -> Self {
        Self(path)
    }
}

// endregion

// region: Config dir-relative path

/// Path relative to the project config directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpConfigDirRelativePath(RelativePathBuf);

impl GtpRelativePath for GtpConfigDirRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

// impl From<&str> for GtpConfigDirRelativePath {
//     fn from(path: &str) -> Self {
//         Self::new(path.into())
//     }
// }

// endregion

// endregion
