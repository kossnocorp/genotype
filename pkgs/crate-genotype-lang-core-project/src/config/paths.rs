use crate::prelude::internal::*;

impl<'a, LangConfig: GtpLangConfig> GtlConfig<'a, LangConfig> {
    /// Returns owned package directory path, i.e. "dist/rs".
    pub fn pkg_dir_path(&'a self) -> GtpPkgDirPath {
        self.project_paths
            .dist
            .join_as_cwd_relative_path(&self.lang_config.dist_relative_pkg_path())
            .into()
    }

    /// Returns owned package target file path, i.e. "dist/rs/.gitignore".
    pub fn pkg_file_path(&'a self, path: &GtpPkgDirRelativePath) -> GtpTargetFilePath {
        self.pkg_dir_path().join_as_cwd_relative_path(path).into()
    }

    /// Returns owned package source path, i.e. "dist/rs/src".
    pub fn pkg_src_path(&'a self) -> GtpPkgSrcDirPath {
        if self.package_enabled {
            self.pkg_dir_path()
                .join_as_cwd_relative_path(&self.lang_config.pkg_dir_relative_src_dir_path())
                .into()
        } else {
            GtpPkgSrcDirPath::from_cwd_relative_path(
                self.pkg_dir_path().cwd_relative_path().clone(),
            )
        }
    }

    /// Returns owned package src file path, i.e. "dist/rs/src/lib.rs".
    pub fn pkg_src_file_path(&'a self, path: &GtpPkgSrcDirRelativePath) -> GtpTargetFilePath {
        self.pkg_src_path().join_as_cwd_relative_path(path).into()
    }

    /// Returns owned package relative source path, i.e. "src".
    pub fn pkg_relative_src_path(&'a self) -> GtpPkgDirRelativePath {
        if self.package_enabled {
            GtpPkgDirRelativePath::new(
                self.lang_config
                    .pkg_dir_relative_src_dir_path()
                    .relative_path()
                    .clone(),
            )
        } else {
            "".into()
        }
    }

    /// Returns owned package source relative file path, i.e. "src/lib.rs".
    pub fn pkg_relative_src_file_path(&'a self, path: &RelativePathBuf) -> GtpPkgDirRelativePath {
        self.pkg_relative_src_path().join_relative_path(path)
    }

    /// Returns owned package target file path from the given source path, i.e.
    /// "dist/rs/src/lib.rs" from "src/lib.rs".
    pub fn module_target_file_path(
        &self,
        source_path: &GtpModulePath,
    ) -> Result<GtpTargetFilePath, GtlConfigError> {
        source_path
            .to_module_id(&self.project_paths.src)
            .map(|module_id|
                // e.g., "module/name" -> "module/name.rs"
                self.lang_config.pkg_src_dir_relative_module_path(&module_id))
            .map(|pkg_src_dir_relative_path|
                // e.g., "src/module/name.rs"
                self.lang_config
                    .pkg_dir_relative_src_dir_path() // e.g., "src"
                    .join_as_pkg_dir_relative_path(&pkg_src_dir_relative_path))
            .map(|pkg_dir_relative_path|
                // e.g., "rs/src/module/name.rs"
                self.lang_config
                    .dist_relative_pkg_path() // e.g., "rs"
                    .join_as_dist_dir_relative_path(&pkg_dir_relative_path))
            .map(|dist_dir_relative_path| {
                // e.g., "dist/rs/src/module/name.rs"
                Ok(self
                    .project_paths
                    .dist // e.g., "dist"
                    .join_as_cwd_relative_path(&dist_dir_relative_path)
                    .into())
            })
            .map_err(|err| GtlConfigError::ResolveTargetModulePath {
                source_path: source_path.clone(),
                message: format!("{err}"),
            })?
    }
}
