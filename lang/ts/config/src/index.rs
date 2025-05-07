use genotype_lang_core_config::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfig {
    #[serde(flatten)]
    pub common: GtlConfigCommon<TsPackagePath>,
}

impl GtlConfig for TsConfig {
    type PackagePath = TsPackagePath;

    fn common(&self) -> &GtlConfigCommon<Self::PackagePath> {
        &self.common
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct TsPackagePath(PathBuf);

impl GtlConfigPackagePathSetting for TsPackagePath {
    const DEFAULT: &'static str = "ts";

    fn pathbuf<'a>(&'a self) -> &'a PathBuf {
        &self.0
    }
}

impl Default for TsPackagePath {
    fn default() -> Self {
        TsPackagePath(Self::default_pathbuf())
    }
}
