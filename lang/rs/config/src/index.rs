use crate::*;
use genotype_lang_core_config::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RsConfig {
    #[serde(flatten)]
    pub lang: RsConfigLang,
    #[serde(flatten)]
    pub common: GtlConfigCommon<RsPackagePath>,
}

impl GtlConfig for RsConfig {
    type PackagePath = RsPackagePath;

    fn common(&self) -> &GtlConfigCommon<Self::PackagePath> {
        &self.common
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct RsPackagePath(PathBuf);

impl GtlConfigPackagePathSetting for RsPackagePath {
    const DEFAULT: &'static str = "rs";

    fn pathbuf<'a>(&'a self) -> &'a PathBuf {
        &self.0
    }
}

impl Default for RsPackagePath {
    fn default() -> Self {
        RsPackagePath(Self::default_pathbuf())
    }
}
