//! Package-related paths. It is the forth level of paths, relative to [GtpDistDirPath].
//!
//! Package path is a path to the target package directory, e.g., `"<root>/dist/rs"`.  It is
//! the base path for the pkg-related paths:
//!
//! - [super::pkg_src]
//!
//! Traits:
//!
//! - [GtpPkgPathWrapper]: Trait for paths that wraps [GtpPkgDirRelativePath] and can be converted to it.
//!
//! Types:
//!
//! - [GtpPkgDirPath]: Package path relative to the working directory. It encloses [GtpDistDirRelativePath].
//! - [GtpPkgDirRelativePath]: Path relative to the target package directory. It encloses [RelativePathBuf].

use crate::prelude::internal::*;

// region: Traits

// region: Pkg dir wrapper path trait

pub trait GtpPkgPathWrapper: GtpRelativePath {
    fn pkg_path(&self) -> &GtpPkgDirRelativePath;
}

// endregion

// region: Types

// region: Package dir path

/// Package path relative to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpPkgDirPath(GtpDistDirRelativePath);

impl GtpRelativePath for GtpPkgDirPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpDistDirRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtpCwdRelativeDirPathWrapper<GtpPkgDirRelativePath> for GtpPkgDirPath {}

// endregion

// region: Package dir-relative path

/// Path relative to [GtpPkgDirPath].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpPkgDirRelativePath(RelativePathBuf);

impl GtpRelativePath for GtpPkgDirRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

// impl From<&str> for GtpPkgPath {
//     fn from(path: &str) -> Self {
//         Self::new(path.into())
//     }
// }

// endregion

// endregion
