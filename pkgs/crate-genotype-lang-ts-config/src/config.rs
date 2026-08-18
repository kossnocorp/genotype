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
        let module_path = self.lang.naming.format_module_path(&module_id.0);
        GtpPkgSrcDirRelativePath::from_str(&format!("{module_path}.ts"))
    }

    fn default_pkg_dir_path(&self) -> GtpDistDirRelativePkgDirPath {
        "ts".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_module_path_naming() {
        let config = TsConfig {
            lang: TsConfigLang {
                naming: TsConfigNaming {
                    source_file: GtpLangConfigNamingCase::SnakeCase,
                    source_dir: Some(GtpLangConfigNamingCase::KebabCase),
                },
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(
            config.pkg_src_dir_relative_module_path(&GtModuleId("ShopGoods/OrderItem".into())),
            "shop-goods/order_item.ts".into()
        );
    }
}
