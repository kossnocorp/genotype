use crate::prelude::internal::*;

mod paths;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpPkgConfig<'a, LangConfig: GtlConfig> {
    /// Dist directory relative to the working directory.
    pub dist: &'a GtpDistDirPath,
    /// Global package version used as language manifest default.
    pub version: Option<&'a Version>,
    /// Global package generation mode.
    pub package: bool,
    /// Target language config.
    pub target: &'a LangConfig,
}

impl<'a, LangConfig: GtlConfig> GtpPkgConfig<'a, LangConfig> {
    pub fn new(
        dist: &'a GtpDistDirPath,
        target: &'a LangConfig,
        version: Option<&'a Version>,
        package: bool,
    ) -> Self {
        Self {
            dist,
            version,
            package,
            target,
        }
    }

    pub fn package_enabled(&self) -> bool {
        self.target.package().unwrap_or(self.package)
    }
}

pub enum GtpConfigLangIdent {
    Py,
    Rs,
    Ts,
}
