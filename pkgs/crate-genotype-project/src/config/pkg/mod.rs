use crate::prelude::internal::*;

mod paths;

impl<'a> GtpConfig {
    pub fn pkg_config_py(&'a self) -> GtpConfigPkg<'a, PyConfig> {
        GtpConfigPkg::new(self.dist_path(), &self.py, self.version.as_ref())
    }

    pub fn pkg_config_rs(&'a self) -> GtpConfigPkg<'a, RsConfig> {
        GtpConfigPkg::new(self.dist_path(), &self.rs, self.version.as_ref())
    }

    pub fn pkg_config_ts(&'a self) -> GtpConfigPkg<'a, TsConfig> {
        GtpConfigPkg::new(self.dist_path(), &self.ts, self.version.as_ref())
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpConfigPkg<'a, LangConfig: GtlConfig> {
    /// Dist directory relative to the working directory.
    pub dist: GtpDistDirPath,
    /// Global package version used as language manifest default.
    pub version: Option<&'a Version>,
    /// Target language config.
    pub target: &'a LangConfig,
}

impl<'a, LangConfig: GtlConfig> GtpConfigPkg<'a, LangConfig> {
    pub fn new(dist: GtpDistDirPath, target: &'a LangConfig, version: Option<&'a Version>) -> Self {
        Self {
            dist,
            version,
            target,
        }
    }
}

pub enum GtpConfigLangIdent {
    Py,
    Rs,
    Ts,
}
