use crate::prelude::internal::*;

impl<Lang: GtpLangConfig> GtpPkgConfig<'_, Lang> {
    /// Returns owned package directory path, i.e. "dist/rs".
    pub fn pkg_dir_path(&self) -> GtpPkgDirPath {
        self.dist
            .join_as_cwd_relative_path(&self.target.dist_relative_pkg_path())
            .into()
    }

    /// Returns owned package target file path, i.e. "dist/rs/.gitignore".
    pub fn pkg_file_path(&self, path: &GtpPkgDirRelativePath) -> GtpTargetFilePath {
        self.pkg_dir_path().join_as_cwd_relative_path(path).into()
    }

    /// Returns owned package source path, i.e. "dist/rs/src".
    pub fn pkg_src_path(&self) -> GtpPkgSrcDirPath {
        if self.package_enabled() {
            self.pkg_dir_path()
                .join_as_cwd_relative_path(&self.target.pkg_dir_relative_src_dir_path())
                .into()
        } else {
            GtpPkgSrcDirPath::from_cwd_relative_path(
                self.pkg_dir_path().cwd_relative_path().clone(),
            )
        }
    }

    /// Returns owned package src file path, i.e. "dist/rs/src/lib.rs".
    pub fn pkg_src_file_path(&self, path: &GtpPkgSrcDirRelativePath) -> GtpTargetFilePath {
        self.pkg_src_path().join_as_cwd_relative_path(path).into()
    }

    /// Returns owned package relative source path, i.e. "src".
    pub fn pkg_relative_src_path(&self) -> GtpPkgDirRelativePath {
        if self.package_enabled() {
            GtpPkgDirRelativePath::new(
                self.target
                    .pkg_dir_relative_src_dir_path()
                    .relative_path()
                    .clone(),
            )
        } else {
            "".into()
        }
    }

    /// Returns owned package source relative file path, i.e. "src/lib.rs".
    pub fn pkg_relative_src_file_path(&self, path: &RelativePathBuf) -> GtpPkgDirRelativePath {
        self.pkg_relative_src_path().join_relative_path(path)
    }
}
