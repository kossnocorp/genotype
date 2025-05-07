use crate::lang::PyConfigLang;
use genotype_lang_core_config::*;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PyConfig {
    pub module: String,
    #[serde(flatten)]
    pub lang: PyConfigLang,
    #[serde(flatten)]
    pub common: GtlConfigCommon<PyPkgPath>,
}

impl GtlConfig for PyConfig {
    type PkgPath = PyPkgPath;

    fn common(&self) -> &GtlConfigCommon<Self::PkgPath> {
        &self.common
    }

    fn src_dir_name<'a>(&'a self) -> &'a str {
        &self.module
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct PyPkgPath(RelativePathBuf);

impl GtlConfigPkgPathSetting for PyPkgPath {
    const DEFAULT: &'static str = "py";

    fn relative_path<'a>(&'a self) -> &'a RelativePathBuf {
        &self.0
    }
}

impl Default for PyPkgPath {
    fn default() -> Self {
        PyPkgPath(Self::default_relative_path())
    }
}
