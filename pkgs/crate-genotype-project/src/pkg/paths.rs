use crate::prelude::internal::*;

impl<Lang: GtlConfig> GtpPkgConfig<'_, Lang> {
    /// Returns owned package directory path, i.e. "dist/rs".
    pub fn pkg_path(&self) -> GtpPkgDirPath {
        self.dist
            .join_as_cwd_relative_path(self.target.dist_relative_pkg_path())
            .into()
    }

    /// Returns owned package file path, i.e. "dist/rs/.gitignore".
    pub fn pkg_file_path(&self, path: &GtpPkgDirRelativePath) -> GtpCwdRelativePath {
        self.pkg_path().join_as_cwd_relative_path(path)
    }

    /// Returns owned package source path, i.e. "dist/rs/src".
    pub fn pkg_src_path(&self) -> GtpPkgSrcDirPath {
        if self.package_enabled() {
            self.pkg_path()
                .join_as_cwd_relative_path(&self.target.src_dir_name())
                .into()
        } else {
            GtpPkgSrcDirPath::from_cwd_relative_path(self.pkg_path().cwd_relative_path().clone())
        }
    }

    /// Returns owned package src file path, i.e. "dist/rs/src/lib.rs".
    pub fn pkg_src_file_path(&self, path: &GtpPkgSrcDirRelativePath) -> GtpCwdRelativePath {
        self.pkg_src_path().join_as_cwd_relative_path(path)
    }

    /// Returns owned package relative source path, i.e. "src".
    pub fn pkg_relative_src_path(&self) -> GtpPkgDirRelativePath {
        if self.package_enabled() {
            self.target.src_dir_name().into()
        } else {
            "".into()
        }
    }

    /// Returns owned package source relative file path, i.e. "src/lib.rs".
    pub fn pkg_relative_src_file_path(&self, path: &RelativePathBuf) -> GtpPkgDirRelativePath {
        self.pkg_relative_src_path().join_relative_path(path)
    }
}
