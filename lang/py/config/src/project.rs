use crate::lang::PyConfigLang;
use genotype_lang_core_config::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PyConfig {
    pub module: String,
    #[serde(flatten)]
    pub lang: PyConfigLang,
    #[serde(flatten)]
    pub common: GtlConfigCommon<PyConfigOut>,
}

impl GtlConfig for PyConfig {
    type Out = PyConfigOut;

    fn common(&self) -> &GtlConfigCommon<Self::Out> {
        &self.common
    }

    fn src_dir_name<'a>(&'a self) -> &'a str {
        &self.module
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct PyConfigOut(PathBuf);

impl GtlConfigOut for PyConfigOut {
    const DEFAULT_OUT: &'static str = "py";

    fn as_path<'a>(&'a self) -> &'a PathBuf {
        &self.0
    }
}

impl Default for PyConfigOut {
    fn default() -> Self {
        PyConfigOut(Self::default_out())
    }
}
