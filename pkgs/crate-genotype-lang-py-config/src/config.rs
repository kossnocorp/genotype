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

impl PyConfig {
    pub fn format_module_path(path: &str) -> String {
        GtpLangConfigNamingCase::format_file_path(
            path,
            GtpLangConfigNamingCase::SnakeCase,
            GtpLangConfigNamingCase::SnakeCase,
        )
    }
}

impl GtpLangConfig for PyConfig {
    fn common(&self) -> &GtpLangConfigCommon {
        &self.common
    }

    fn pkg_dir_relative_src_dir_path(&self) -> GtpPkgDirRelativePkgSrcDirPath {
        self.module.as_str().into()
    }

    fn pkg_src_dir_relative_module_path(&self, module_id: &GtModuleId) -> GtpPkgSrcDirRelativePath {
        let module_path = Self::format_module_path(&module_id.0);
        GtpPkgSrcDirRelativePath::from_str(&format!("{module_path}.py"))
    }

    fn default_pkg_dir_path(&self) -> GtpDistDirRelativePkgDirPath {
        "py".into()
    }

    fn comment_prefix(&self) -> &'static str {
        "#"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_module_path_naming() {
        assert_eq!(
            PyConfig::default()
                .pkg_src_dir_relative_module_path(&GtModuleId("ShopGoods/OrderItem".into())),
            "shop_goods/order_item.py".into()
        );
    }
}
