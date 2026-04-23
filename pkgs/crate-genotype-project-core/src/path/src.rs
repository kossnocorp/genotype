//! Src-related paths. It is the third level of paths, relative to [GtpRootDirPath].
//!
//! Src dir path is defined in the config and by default equals to "src". It is the base path for
//! the src-related paths:
//!
//! - [super::entry]
//! - [super::module]
//!
//! Traits:
//!
//! - [GtpSrcPathWrapper]: Trait for paths that wraps [GtpSrcDirRelativePath] and can be converted to it.
//!
//! Types:
//!
//! - [GtpSrcDirPath]: Src dir path relative to the cwd path. It encloses [GtpCwdRelativePath].
//! - [GtpSrcDirRelativePath]: Path relative to the src directory. It encloses [RelativePathBuf].

use crate::prelude::internal::*;

// region: Traits

// region: Src dir wrapper path trait

pub trait GtpSrcPathWrapper: GtpRelativePath {
    fn src_path(&self) -> &GtpSrcDirRelativePath;
}

// endregion

// endregion

// region: Types

// region: Src dir path

/// Src dir path relative to the cwd path.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpSrcDirPath(GtpCwdRelativePath);

impl GtpRelativePath for GtpSrcDirPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpCwdRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtpCwdRelativeDirPathWrapper<GtpSrcDirRelativePath> for GtpSrcDirPath {}

impl From<GtpCwdRelativePath> for GtpSrcDirPath {
    fn from(path: GtpCwdRelativePath) -> Self {
        Self(path)
    }
}

// endregion

// region: Src dir-relative path

/// Path relative to [GtpSrcDirPath].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpSrcDirRelativePath(RelativePathBuf);

impl GtpRelativePath for GtpSrcDirRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

// impl From<&str> for GtpSrcDirRelativePath {
//     fn from(path: &str) -> Self {
//         Self::new(path.into())
//     }
// }

// endregion

// endregion
