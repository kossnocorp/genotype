use crate::prelude::internal::*;

mod edition;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RsConfig {
    #[serde(flatten)]
    pub lang: RsConfigLang,
    #[serde(flatten)]
    pub common: GtpLangConfigCommon, //<RsPkgPath>,
}

impl RsConfig {
    pub fn format_module_path(path: &str) -> String {
        GtpLangConfigNamingCase::format_file_path(
            path,
            GtpLangConfigNamingCase::SnakeCase,
            GtpLangConfigNamingCase::SnakeCase,
        )
    }
}

impl GtpLangConfig for RsConfig {
    // type PkgPath = RsPkgPath;

    fn common(&self) -> &GtpLangConfigCommon {
        // <Self::PkgPath> {
        &self.common
    }

    fn pkg_src_dir_relative_module_path(&self, module_id: &GtModuleId) -> GtpPkgSrcDirRelativePath {
        let module_path = Self::format_module_path(&module_id.0);
        GtpPkgSrcDirRelativePath::from_str(&format!("{module_path}.rs"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_module_path_naming() {
        assert_eq!(
            RsConfig::default()
                .pkg_src_dir_relative_module_path(&GtModuleId("ShopGoods/OrderItem".into())),
            "shop_goods/order_item.rs".into()
        );
    }
}
