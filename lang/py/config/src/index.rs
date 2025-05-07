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
    pub common: GtlConfigCommon<PyPackagePath>,
}

impl GtlConfig for PyConfig {
    type PackagePath = PyPackagePath;

    fn common(&self) -> &GtlConfigCommon<Self::PackagePath> {
        &self.common
    }

    fn src_dir_name<'a>(&'a self) -> &'a str {
        &self.module
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(transparent)]
pub struct PyPackagePath(PathBuf);

impl GtlConfigPackagePathSetting for PyPackagePath {
    const DEFAULT: &'static str = "py";

    fn pathbuf<'a>(&'a self) -> &'a PathBuf {
        &self.0
    }
}

impl Default for PyPackagePath {
    fn default() -> Self {
        PyPackagePath(Self::default_pathbuf())
    }
}
