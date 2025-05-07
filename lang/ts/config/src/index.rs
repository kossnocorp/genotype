use genotype_lang_core_config::*;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfig {
    #[serde(flatten)]
    pub common: GtlConfigCommon<TsPkgPath>,
}

impl GtlConfig for TsConfig {
    type PkgPath = TsPkgPath;

    fn common(&self) -> &GtlConfigCommon<Self::PkgPath> {
        &self.common
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct TsPkgPath(RelativePathBuf);

impl GtlConfigPkgPathSetting for TsPkgPath {
    const DEFAULT: &'static str = "ts";

    fn relative_path<'a>(&'a self) -> &'a RelativePathBuf {
        &self.0
    }
}

impl Default for TsPkgPath {
    fn default() -> Self {
        TsPkgPath(Self::default_relative_path())
    }
}
