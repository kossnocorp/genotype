use crate::prelude::internal::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfig {
    #[serde(flatten)]
    pub common: GtpLangConfigCommon,

    #[serde(flatten)]
    pub lang: TsConfigLang,
}

impl GtpLangConfig for TsConfig {
    fn common(&self) -> &GtpLangConfigCommon {
        &self.common
    }

    fn pkg_src_dir_relative_module_path(&self, module_id: &GtModuleId) -> GtpPkgSrcDirRelativePath {
        GtpPkgSrcDirRelativePath::from_str(&format!("{}.ts", module_id.0.as_ref()))
    }

    fn default_pkg_dir_path(&self) -> GtpDistDirRelativePkgDirPath {
        "ts".into()
    }
}
