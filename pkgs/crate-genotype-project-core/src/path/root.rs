//! Root-related paths. It is the second level of paths, relative to [GtpConfigDirPath].
//!
//! Root path is defined in the config and by default equals to the config dir. It is the base path
//! for the root-related paths:
//!
//! - [super::src]
//! - [super::dist]
//!
//! Traits:
//!
//! - [GtpRootDirRelativePathWrapper]: Trait for paths that wraps [GtpRootDirRelativePath] and can be converted to it.
//!
//! Types:
//!
//! - [GtpRootDirPath]: Root dir path relative to the config dir. It encloses [GtpConfigDirRelativePath].
//! - [GtpRootDirRelativePath]: Path relative to the root dir. It encloses [RelativePathBuf].

use crate::prelude::internal::*;

// region: Traits

// region: Root dir-relative path wrapper path trait

pub trait GtpRootDirRelativePathWrapper: GtpRelativePath {
    fn to_root_path(&self) -> GtpRootDirRelativePath;

    fn to_cwd_path(
        &self,
        config_dir: &GtpConfigDirPath,
        root_dir_path: &GtpRootDirPath,
    ) -> GtpCwdRelativePath {
        GtpCwdRelativePath::new(
            config_dir.relative_path().join(
                root_dir_path
                    .relative_path()
                    .join(self.to_root_path().relative_path()),
            ),
        )
    }
}

// endregion

// endregion

// region: Types

// region: Root dir path

/// Root path relative to config dir. By default, it equals to ".", but it
/// can be configured to a different path.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpRootDirPath(GtpConfigDirRelativePath);

impl GtpRootDirPath {
    pub fn join_as_root_dir_relative_path(
        &self,
        path: &GtpRootDirRelativePath,
    ) -> GtpRootDirRelativePath {
        GtpRootDirRelativePath::new(self.relative_path().join(path.relative_path()))
    }
}

impl GtpRelativePath for GtpRootDirPath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpConfigDirRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtpRootDirRelativePathWrapper for GtpRootDirPath {
    fn to_root_path(&self) -> GtpRootDirRelativePath {
        GtpRootDirRelativePath::new(".".into())
    }
}

impl Into<GtpRootDirRelativePath> for GtpRootDirPath {
    fn into(self) -> GtpRootDirRelativePath {
        GtpRootDirRelativePath::new(self.0.relative_path().to_owned())
    }
}

// endregion

// region: Root dir-relative path

/// Path relative to [GtpRootDirPath].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpRootDirRelativePath(RelativePathBuf);

impl GtpRelativePath for GtpRootDirRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

// impl From<&str> for GtpRootDirRelativePath {
//     fn from(path: &str) -> Self {
//         Self::new(path.into())
//     }
// }

// endregion

// endregion
