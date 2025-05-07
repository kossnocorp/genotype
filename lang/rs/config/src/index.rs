use crate::*;
use genotype_lang_core_config::*;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RsConfig {
    #[serde(flatten)]
    pub lang: RsConfigLang,
    #[serde(flatten)]
    pub common: GtlConfigCommon<RsPkgPath>,
}

impl GtlConfig for RsConfig {
    type PkgPath = RsPkgPath;

    fn common(&self) -> &GtlConfigCommon<Self::PkgPath> {
        &self.common
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct RsPkgPath(RelativePathBuf);

impl GtlConfigPkgPathSetting for RsPkgPath {
    const DEFAULT: &'static str = "rs";

    fn relative_path<'a>(&'a self) -> &'a RelativePathBuf {
        &self.0
    }
}

impl Default for RsPkgPath {
    fn default() -> Self {
        RsPkgPath(Self::default_relative_path())
    }
}
