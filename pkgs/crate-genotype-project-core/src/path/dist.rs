//! Dist-related paths. It is the third level of paths, relative to [GtpRootDirRelativePath].
//!
//! Dist dir path is defined in the config and by default equals to "dist". It is the base path for
//! the dist-related paths:
//!
//! - [super::pkg]
//!
//! Traits:
//!
//! - [GtpDistPathWrapper]: Trait for paths that wraps [GtpDistDirRelativePath] and can be converted to it.
//!
//! Types:
//!
//! - [GtpDistDirPath]: Dist dir path relative to the cwd path. It encloses [GtpDistDirRelativePath].
//! - [GtpDistDirRelativePath]: Path relative to the dist directory. It encloses [RelativePathBuf].

use crate::prelude::internal::*;

// region: Traits

// region: Dist dir wrapper path trait

pub trait GtpDistPathWrapper: GtpRelativePath {
    fn dist_path(&self) -> &GtpDistDirRelativePath;
}

// endregion

// region: Types

// region: Dist dir path

/// Dist dir path relative to the root dir.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpDistDirPath(GtpRootDirRelativePath);

impl GtpRelativePath for GtpDistDirPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpRootDirRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtpCwdRelativeDirPathWrapper<GtpDistDirRelativePath> for GtpDistDirPath {}

// impl From<GtpRootPath> for GtpDistDirPath {
//     fn from(path: GtpRootPath) -> Self {
//         Self(GtpCwdPath::from(path))
//     }
// }

// endregion

// region: Dist dir-relative path

/// Path relative to [GtpDistDirPath].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpDistDirRelativePath(RelativePathBuf);

impl GtpRelativePath for GtpDistDirRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

// impl From<&str> for GtpDistDirRelativePath {
//     fn from(path: &str) -> Self {
//         Self::new(path.into())
//     }
// }

// endregion

// endregion
