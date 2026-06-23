use crate::prelude::internal::*;

mod common;
pub use common::*;

pub trait GtpLangConfig {
    /// Returns the common language configuration. The target language configuration overrides
    /// the method to provide access to the config struct.
    fn common(&self) -> &GtpLangConfigCommon;

    /// Target-specific source directory name. For instance, TypeScript and Rust packages will not
    /// have to override this method, but Python package source directory has the package module
    /// name, i.e. ""
    fn pkg_dir_relative_src_dir_path(&self) -> GtpPkgDirRelativePkgSrcDirPath {
        "src".into()
    }

    /// Returns the target package directory path relative to the dist directory, i.e. "rs".
    fn dist_relative_pkg_path(&self) -> GtpDistDirRelativePkgDirPath {
        self.common()
            .dist.clone()
            .unwrap_or_else(|| self.default_pkg_dir_path())
    }

    /// Generates pkg src dir-relative path from the given module id, e.g., "module/name.rs" from
    /// "module/name". It is must be implemented by the target language config.
    fn pkg_src_dir_relative_module_path(&self, module_id: &GtModuleId) -> GtpPkgSrcDirRelativePath;

    /// Returns the config-defined manifest table. This is used to generate the package manifest,
    /// i.e. `Cargo.toml`.
    fn manifest(&self) -> &Table {
        &self.common().manifest
    }

    /// Returns the target package generation mode override.
    fn package(&self) -> Option<bool> {
        self.common().package
    }

    /// Default dist-relative pkg dir path, e.g., "rs".
    fn default_pkg_dir_path(&self) -> GtpDistDirRelativePkgDirPath;

    fn health_check(
        &self,
        _config_path: &GtpConfigFilePath,
        _package_enabled: bool,
    ) -> Vec<GtNotice> {
        vec![]
    }
}
