//! Module-related paths. It is the fourth level of paths, relative to [GtpSrcDirPath].
//!
//! Module path is the path to Genotype source file. It represents the module id in a project.
//!
//! Types:
//!
//! - [GtpModulePath]: Module path relative to [GtpCwdPath].
//! - [GtpSrcDirRelativeModulePath]: Module path relative to [GtpSrcDirPath].

use crate::prelude::internal::*;

// region: Cwd-relative module path

gtp_cwd_relative_path_wrapper_newtype!(
    /// Module path relative to cwd.
    pub struct GtpModulePath(GtpCwdRelativePath);
);

// endregion

// region: Src dir-relative module path

gtp_relative_path_wrapper_newtype!(
    /// Module path relative to the src directory.
    pub struct GtpSrcDirRelativeModulePath(GtpSrcDirRelativePath);
    parent: GtpSrcDirPath;
);

impl GtpSrcDirRelativeModulePath {
    #[cfg(feature = "parser")]
    pub fn resolve(&self, path: &GtPath) -> GtpSrcDirRelativeModulePath {
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

    /// Transforms the src relative path into a package source relative path. It helps targets
    /// generating the correct path for the package source.
    pub fn to_pkg_src_relative_file_path(&self, ext: &'static str) -> GtpPkgSrcDirRelativePath {
        GtpPkgSrcDirRelativePath::new(self.relative_path().with_extension(ext))
    }
}

#[cfg(feature = "parser")]
impl From<&GtpSrcDirRelativeModulePath> for GtModuleId {
    fn from(path: &GtpSrcDirRelativeModulePath) -> Self {
        path.relative_path().with_extension("").as_str().into()
    }
}

#[cfg(feature = "parser")]
impl From<GtpSrcDirRelativeModulePath> for GtModuleId {
    fn from(path: GtpSrcDirRelativeModulePath) -> Self {
        (&path).into()
    }
}

// endregion
