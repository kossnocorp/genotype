//! Package src-related paths. It is the fifth level of paths, relative to [GtpPkgDirPath].
//!
//! Package src path is a path to the target package src directory, e.g., `"<root>/dist/rs/src"`.
//!
//! Types:
//!
//! - [GtpPkgSrcDirPath]: Package src path relative to the working directory. It encloses
//!   [GtpPkgDirRelativePath].
//! - [GtpPkgSrcDirRelativePath]: Path relative to the target package src directory. It encloses
//!   [RelativePathBuf].

use crate::prelude::internal::*;

// region: Types

// region: Package src dir path

/// Package src path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpPkgSrcDirPath(GtpPkgDirRelativePath);

impl GtpRelativePath for GtpPkgSrcDirPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpPkgDirRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtpPkgPathWrapper for GtpPkgSrcDirPath {
    fn pkg_path(&self) -> &GtpPkgDirRelativePath {
        &self.0
    }
}

// endregion

// region: Package src dir-relative path

/// Path relative to [GtpPkgSrcDirPath].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpPkgSrcDirRelativePath(RelativePathBuf);

impl GtpRelativePath for GtpPkgSrcDirRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

// impl From<&str> for GtpPkgSrcDirRelativePath {
//     fn from(path: &str) -> Self {
//         Self::new(path.into())
//     }
// }

// endregion

// endregion
