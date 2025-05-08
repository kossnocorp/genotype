use crate::prelude::internal::*;

impl<Lang: GtlConfig> GtConfigPkg<'_, Lang> {
    /// Returns owned package directory path, i.e. "dist/rs".
    pub fn pkg_path(&self) -> GtPkgPath {
        self.dist.join(&self.target.dist_relative_pkg_path()).into()
    }

    /// Returns owned package file path, i.e. "dist/rs/.gitignore".
    pub fn pkg_file_path(&self, path: &GtPkgRelativePath) -> GtCwdRelativePath {
        self.pkg_path().join(&path)
    }

    /// Returns owned package source path, i.e. "dist/rs/src".
    pub fn pkg_src_path(&self) -> GtPkgSrcPath {
        self.pkg_path()
            .join(&self.target.src_dir_name().into())
            .into()
    }

    /// Returns owned package src file path, i.e. "dist/rs/src/lib.rs".
    pub fn pkg_src_file_path(&self, path: &GtPkgSrcRelativePath) -> GtCwdRelativePath {
        self.pkg_src_path().join(&path)
    }

    /// Returns owned package relative source path, i.e. "src".
    pub fn pkg_relative_src_path(&self) -> GtPkgRelativePath {
        self.target.src_dir_name().into()
    }
    /// Returns owned package source relative file path, i.e. "src/lib.rs".
    pub fn pkg_relative_src_file_path(&self, path: &RelativePathBuf) -> GtPkgRelativePath {
        self.pkg_relative_src_path().join_path(path)
    }
}
