use crate::prelude::internal::*;

impl<'a> GtProject {
    pub fn pkg_config_py(&'a self) -> GtpPkgConfig<'a, PyConfig> {
        GtpPkgConfig::new(
            &self.paths.dist,
            &self.config.py,
            self.config.version.as_ref(),
        )
    }

    pub fn pkg_config_rs(&'a self) -> GtpPkgConfig<'a, RsConfig> {
        GtpPkgConfig::new(
            &self.paths.dist,
            &self.config.rs,
            self.config.version.as_ref(),
        )
    }

    pub fn pkg_config_ts(&'a self) -> GtpPkgConfig<'a, TsConfig> {
        GtpPkgConfig::new(
            &self.paths.dist,
            &self.config.ts,
            self.config.version.as_ref(),
        )
    }
}
