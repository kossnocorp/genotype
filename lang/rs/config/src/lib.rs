use genotype_lang_core_config::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod lang;
pub use lang::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RsConfig {
    #[serde(flatten)]
    pub lang: RsConfigLang,
    #[serde(flatten)]
    pub common: GtlConfigCommon<RsConfigOut>,
}

impl GtlConfig for RsConfig {
    type Out = RsConfigOut;

    fn common(&self) -> &GtlConfigCommon<Self::Out> {
        &self.common
    }

    fn src_dir_name<'a>(&'a self) -> &'a str {
        "src"
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct RsConfigOut(PathBuf);

impl GtlConfigOut for RsConfigOut {
    const DEFAULT_OUT: &'static str = "rs";

    fn as_path<'a>(&'a self) -> &'a PathBuf {
        &self.0
    }
}

impl Default for RsConfigOut {
    fn default() -> Self {
        RsConfigOut(Self::default_out())
    }
}
