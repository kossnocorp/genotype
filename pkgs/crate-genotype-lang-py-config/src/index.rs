use crate::prelude::internal::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PyConfig {
    #[serde(default)]
    pub module: PyModuleName,
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

    fn src_dir_name(&self) -> &str {
        self.module.as_str()
    }
}

impl GtlConfigHealth for PyConfig {}
