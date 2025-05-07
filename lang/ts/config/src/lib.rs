use genotype_lang_core_config::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfig {
    #[serde(flatten)]
    pub common: GtlConfigCommon<TsConfigOut>,
}

impl GtlConfig for TsConfig {
    type Out = TsConfigOut;

    fn common(&self) -> &GtlConfigCommon<Self::Out> {
        &self.common
    }

    fn src_dir_name<'a>(&'a self) -> &'a str {
        "src"
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct TsConfigOut(PathBuf);

impl GtlConfigOut for TsConfigOut {
    const DEFAULT_OUT: &'static str = "ts";

    fn as_path<'a>(&'a self) -> &'a PathBuf {
        &self.0
    }
}

impl Default for TsConfigOut {
    fn default() -> Self {
        TsConfigOut(Self::default_out())
    }
}
