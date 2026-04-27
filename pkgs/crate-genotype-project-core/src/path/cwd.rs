//! Current working directory (cwd)-related paths. It is the base for the rest of the paths.
//!
//! All file system operations expect either absolute paths or a cwd-relative path. So all paths
//! must be convertible to [GtpCwdRelativePath].
//!
//! Cwd path is the base path for all cwd-relative paths:
//!
//! - [super::config]
//!
//! Types:
//!
//! - [GtpCwdPath]: Absolute canonical cwd path. It encloses absolute [PathBuf].
//! - [GtpCwdRelativePath]: Path relative to [GtpCwdPath]. It encloses [RelativePathBuf].
//!
//! Traits:
//!
//! - [GtpCwdRelativePathWrapper]: Trait for paths that wraps [GtpCwdRelativePath] and can be converted to it.
//! - [GtpDirPath]: Similar to [GtpCwdRelativePathWrapper], but for cwd-relative directory paths.
//! - [GtpParentRelativePath]: Trait for paths that are relative to a cwd-relative parent directory.

use crate::prelude::internal::*;

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

gtp_relative_path_newtype!(
    /// Path relative to cwd.
    pub struct GtpCwdRelativePath;
);

// endregion

// region: Cwd-relative str-wrapper path

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GtpCwdRelativeOrAbsoluteStringPath(String);

impl From<&str> for GtpCwdRelativeOrAbsoluteStringPath {
    fn from(path: &str) -> Self {
        Self(path.to_string())
    }
}

impl TryInto<GtpCwdRelativePath> for &GtpCwdRelativeOrAbsoluteStringPath {
    type Error = miette::Report;

    fn try_into(self) -> Result<GtpCwdRelativePath> {
        let cwd_path = GtpCwdPath::try_new()?;
        let path = PathBuf::from(&self.0);
        let rel_path = if path.is_absolute() {
            path.relative_to(&cwd_path.as_path())
                .map_err(|e| miette!(e)).wrap_err_with(||format!("failed to resolve base path from '{}' relative to current working directory '{}'", path.display(), cwd_path.as_path().display()))?
        } else {
            RelativePathBuf::from_path(&path)
                .map_err(|e| miette!(e))
                .wrap_err_with(|| {
                    format!(
                        "failed to convert base path '{}' to relative path",
                        path.display()
                    )
                })?
        };
        Ok(GtpCwdRelativePath::new(rel_path))
    }
}

impl TryInto<GtpCwdRelativePath> for GtpCwdRelativeOrAbsoluteStringPath {
    type Error = miette::Report;

    fn try_into(self) -> Result<GtpCwdRelativePath> {
        (&self).try_into()
    }
}

impl Display for GtpCwdRelativeOrAbsoluteStringPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// endregion
