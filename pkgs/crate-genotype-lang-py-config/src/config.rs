use crate::prelude::internal::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PyConfig {
    #[serde(default)]
    pub module: PyModuleName,
    #[serde(flatten)]
    pub lang: PyConfigLang,
    #[serde(flatten)]
    pub common: GtpLangConfigCommon,
}

impl GtpLangConfig for PyConfig {
    fn common(&self) -> &GtpLangConfigCommon {
        &self.common
    }

    fn pkg_dir_relative_src_dir_path(&self) -> GtpPkgDirRelativePkgSrcDirPath {
        self.module.as_str().into()
    }

    fn pkg_src_dir_relative_module_path(&self, module_id: &GtModuleId) -> GtpPkgSrcDirRelativePath {
        GtpPkgSrcDirRelativePath::from_str(&format!("{}.py", module_id.0.as_ref()))
    }

    fn default_pkg_dir_path(&self) -> GtpDistDirRelativePkgDirPath {
        "py".into()
    }
}
