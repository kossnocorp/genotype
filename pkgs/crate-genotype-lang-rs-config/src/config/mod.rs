use crate::prelude::internal::*;

mod edition;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RsConfig {
    #[serde(flatten)]
    pub lang: RsConfigLang,
    #[serde(flatten)]
    pub common: GtpLangConfigCommon, //<RsPkgPath>,
}

impl GtpLangConfig for RsConfig {
    // type PkgPath = RsPkgPath;

    fn common(&self) -> &GtpLangConfigCommon {
        // <Self::PkgPath> {
        &self.common
    }

    fn pkg_src_dir_relative_module_path(&self, module_id: &GtModuleId) -> GtpPkgSrcDirRelativePath {
        GtpPkgSrcDirRelativePath::from_str(&format!("{}.rs", module_id.0.as_ref()))
    }

    fn default_pkg_dir_path(&self) -> GtpDistDirRelativePkgDirPath {
        "rs".into()
    }

    fn health_check(
        &self,
        config_path: &GtpConfigFilePath,
        package_enabled: bool,
    ) -> Vec<GtDiagnostic> {
        let mut diagnostics = vec![];

        if let Some(diagnostic) = self.rust_edition_health_check(config_path, package_enabled) {
            diagnostics.push(diagnostic);
        }

        diagnostics
    }
}
