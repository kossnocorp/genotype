use crate::prelude::internal::*;

mod paths;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpPkgConfig<'a, LangConfig: GtlConfig> {
    /// Dist directory relative to the working directory.
    pub dist: &'a GtpDistDirPath,
    /// Global package version used as language manifest default.
    pub version: Option<&'a Version>,
    /// Target language config.
    pub target: &'a LangConfig,
}

impl<'a, LangConfig: GtlConfig> GtpPkgConfig<'a, LangConfig> {
    pub fn new(
        dist: &'a GtpDistDirPath,
        target: &'a LangConfig,
        version: Option<&'a Version>,
    ) -> Self {
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
