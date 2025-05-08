use crate::prelude::internal::*;

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
