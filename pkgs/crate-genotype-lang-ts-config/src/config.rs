use crate::prelude::internal::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TsConfig {
    #[serde(flatten)]
    pub common: GtpLangConfigCommon, // <TsPkgPath>,

    #[serde(flatten)]
    pub lang: TsConfigLang,
}

impl GtpLangConfig for TsConfig {
    // type PkgPath = TsPkgPath;

    fn common(&self) -> &GtpLangConfigCommon {
        //<Self::PkgPath> {
        &self.common
    }

    fn pkg_src_dir_relative_module_path(&self, module_id: &GtModuleId) -> GtpPkgSrcDirRelativePath {
        GtpPkgSrcDirRelativePath::from_str(&format!("{}.ts", module_id.0.as_ref()))
    }

    fn default_pkg_dir_path(&self) -> GtpDistDirRelativePkgDirPath {
        "ts".into()
    }
}
