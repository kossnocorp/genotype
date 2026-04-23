//! Module-related paths. It is the forth level of paths, relative to [GtpSrcDirPath].
//!
//! Module path is the path to Genotype source file. It represents the module id in a project.
//!
//! Types:
//!
//! - [GtpModulePath]: Module path relative to the src directory. It encloses [GtpSrcDirRelativePath].

use crate::prelude::internal::*;

// region: Types

// region: Module path

/// Module path relative to the src directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtpModulePath(GtpSrcDirRelativePath);

impl GtpModulePath {
    #[cfg(feature = "parser")]
    pub fn resolve(&self, path: &GtPath) -> GtpModulePath {
        let parent_path = if let Some(parent) = self.0.relative_path().parent() {
            parent
        } else {
            &RelativePathBuf::from("")
        };
        Self::new(
            parent_path
                .join_normalized(path.source_str())
                .with_extension("type"),
        )
    }

    pub fn as_src_relative_path(self) -> GtpSrcDirRelativePath {
        self.0
    }

    /// Transforms the src relative path into a package source relative path. It helps targets
    /// generating the correct path for the package source.
    pub fn to_pkg_src_relative_path(&self, ext: &'static str) -> GtpPkgSrcDirRelativePath {
        GtpPkgSrcDirRelativePath::new(self.relative_path().with_extension(ext))
    }
}

impl GtpRelativePath for GtpModulePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtpSrcDirRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl GtpSrcPathWrapper for GtpModulePath {
    fn src_path(&self) -> &GtpSrcDirRelativePath {
        &self.0
    }
}

impl From<GtpModulePath> for GtpSrcDirRelativePath {
    fn from(path: GtpModulePath) -> Self {
        path.0
    }
}

// impl From<&str> for GtpModulePath {
//     fn from(path: &str) -> Self {
//         Self::new(path.into())
//     }
// }

#[cfg(feature = "parser")]
impl From<&GtpModulePath> for GtModuleId {
    fn from(path: &GtpModulePath) -> Self {
        path.relative_path().with_extension("").as_str().into()
    }
}

#[cfg(feature = "parser")]
impl From<GtpModulePath> for GtModuleId {
    fn from(path: GtpModulePath) -> Self {
        (&path).into()
    }
}

// endregion

// endregion
