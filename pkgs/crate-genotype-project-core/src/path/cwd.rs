//! Current working directory (cwd)-related paths. It is the base for the rest of the paths.
//!
//! All file system operations expect either absolute paths or a cwd-relative path. So all paths
//! must be convertible to [GtpCwdRelativePath].
//!
//! Cwd path is the base path for all cwd-relative paths:
//!
//! - [super::config]
//!
//! Traits:
//!
//! - [GtpCwdRelativePathWrapper]: Trait for paths that wraps [GtpCwdRelativePath] and can be converted to it.
//! - [GtpCwdRelativeDirPathWrapper]: Similar to [GtpCwdRelativePathWrapper], but for dir-guaranteed paths.
//!
//! Types:
//!
//! - [GtpCwdPath]: Absolute canonical cwd path. It encloses absolute [PathBuf].
//! - [GtpCwdRelativePath]: Path relative to [GtpCwdPath]. It encloses [RelativePathBuf].

use crate::prelude::internal::*;

// region: Traits

// region: Cwd-relative path wrapper trait

pub trait GtpCwdRelativePathWrapper: GtpRelativePath {
    fn to_module_path(&self, src_dir_path: &GtpSrcDirPath) -> Result<GtpModulePath> {
        let rel_path = self
            .relative_path()
            .strip_prefix(src_dir_path.relative_path())
            .map_err(|_| {
                miette!(
                    "path '{}' is not under src directory '{}'",
                    self.display(),
                    src_dir_path.display()
                )
            })?;
        Ok(GtpModulePath::new(rel_path.to_owned()))
    }
}

// endregion

// region: Cwd-relative dir path wrapper trait

pub trait GtpCwdRelativeDirPathWrapper<ChildPath: GtpRelativePath>: GtpRelativePath {
    fn join_as_cwd_relative_path(&self, path: &ChildPath) -> GtpCwdRelativePath {
        GtpCwdRelativePath::new(self.relative_path().join_normalized(path.relative_path()))
    }
}

// endregion

// endregion

// region: Types

// region: Absolute cwd path

/// Absolute canonical path to the working directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpCwdPath(PathBuf);

impl GtpCwdPath {
    pub fn try_new() -> Result<Self> {
        let path = std::env::current_dir()
            .and_then(|path| path.canonicalize())
            .map_err(|e| miette!(e))
            .wrap_err("failed to get current working directory")?;
        Ok(Self(path))
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

// endregion

// region: Cwd-relative path

/// Path relative to [GtpCwdPath].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpCwdRelativePath(RelativePathBuf);

impl GtpRelativePath for GtpCwdRelativePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(path.normalize())
    }

    fn relative_path(&self) -> &RelativePathBuf {
        &self.0
    }
}

impl GtpCwdRelativePathWrapper for GtpCwdRelativePath {}

// impl From<&str> for GtpCwdRelativePath {
//     fn from(path: &str) -> Self {
//         Self::new(path.into())
//     }
// }

// endregion

// endregion
