use crate::prelude::internal::*;

mod paths;

impl<'a> GtConfig {
    pub fn pkg_config_py(&'a self) -> GtConfigPkg<'a, PyConfig> {
        GtConfigPkg::new(self.dist_path(), &self.py)
    }

    pub fn pkg_config_rs(&'a self) -> GtConfigPkg<'a, RsConfig> {
        GtConfigPkg::new(self.dist_path(), &self.rs)
    }

    pub fn pkg_config_ts(&'a self) -> GtConfigPkg<'a, TsConfig> {
        GtConfigPkg::new(self.dist_path(), &self.ts)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GtConfigPkg<'a, LangConfig: GtlConfig> {
    /// Dist directory relative to the working directory.
    pub dist: GtDistPath,
    /// Target language config.
    pub target: &'a LangConfig,
}

impl<'a, LangConfig: GtlConfig> GtConfigPkg<'a, LangConfig> {
    pub fn new(dist: GtDistPath, target: &'a LangConfig) -> Self {
        Self { dist, target }
    }
}

pub enum GtConfigLangIdent {
    Py,
    Rs,
    Ts,
}
