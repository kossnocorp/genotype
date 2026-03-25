use semver::Version;
use serde::Serialize;

use crate::prelude::internal::*;

mod paths;

impl<'a> GtConfig {
    pub fn pkg_config_py(&'a self) -> GtConfigPkg<'a, PyConfig> {
        GtConfigPkg::new(self.dist_path(), &self.py, self.version.as_ref())
    }

    pub fn pkg_config_rs(&'a self) -> GtConfigPkg<'a, RsConfig> {
        GtConfigPkg::new(self.dist_path(), &self.rs, self.version.as_ref())
    }

    pub fn pkg_config_ts(&'a self) -> GtConfigPkg<'a, TsConfig> {
        GtConfigPkg::new(self.dist_path(), &self.ts, self.version.as_ref())
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtConfigPkg<'a, LangConfig: GtlConfig> {
    /// Dist directory relative to the working directory.
    pub dist: GtDistPath,
    /// Global package version used as language manifest default.
    pub version: Option<&'a Version>,
    /// Target language config.
    pub target: &'a LangConfig,
}

impl<'a, LangConfig: GtlConfig> GtConfigPkg<'a, LangConfig> {
    pub fn new(dist: GtDistPath, target: &'a LangConfig, version: Option<&'a Version>) -> Self {
        Self {
            dist,
            version,
            target,
        }
    }
}

pub enum GtConfigLangIdent {
    Py,
    Rs,
    Ts,
}
