use crate::prelude::internal::*;
use toml::Table;

mod notice;
pub use notice::*;

mod health;
pub use health::*;

pub trait GtlConfig: GtlConfigHealth + Default {
    /// The target package path setting type.
    type PkgPath: GtlConfigPkgPathSetting;

    /// Returns the common language configuration. The target language configuration overrides
    /// the method to provide acess to the config struct.
    fn common(&self) -> &GtlConfigCommon<Self::PkgPath>;

    /// Target-specific source directory name. For instance, TypeScript and Rust packages will not
    /// have to override this method, but Python package source directory has the package module
    /// name, i.e. ""
    fn src_dir_name(&self) -> &str {
        "src"
    }

    /// Returns the target package directory path relative to the dist directory, i.e. "rs".
    fn dist_relative_pkg_path(&self) -> &GtpDistDirRelativePath {
        self.common().out.path()
    }

    /// Returns the config-defined manifest table. This is used to generate the package manifest,
    /// i.e. `Cargo.toml`.
    fn manifest(&self) -> &Table {
        &self.common().manifest
    }
}
