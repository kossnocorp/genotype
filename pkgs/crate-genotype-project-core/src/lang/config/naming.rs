use crate::prelude::internal::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Default)]
pub enum GtpLangConfigNamingCase {
    #[default]
    #[serde(rename = "camelCase")]
    CamelCase,
    #[serde(rename = "PascalCase")]
    PascalCase,
    #[serde(rename = "snake_case")]
    SnakeCase,
    #[serde(rename = "kebab-case")]
    KebabCase,
}

impl GtpLangConfigNamingCase {
    pub fn format_component(&self, name: &str) -> String {
        match self {
            Self::CamelCase => name.to_lower_camel_case(),
            Self::PascalCase => name.to_pascal_case(),
            Self::SnakeCase => name.to_snake_case(),
            Self::KebabCase => name.to_kebab_case(),
        }
    }

    pub fn format_file_path(
        path: &str,
        source_dir: GtpLangConfigNamingCase,
        source_file: GtpLangConfigNamingCase,
    ) -> String {
        let components = path.split('/').collect::<Vec<_>>();
        let last_name_index = components
            .iter()
            .rposition(|component| !matches!(*component, "" | "." | ".." | "~"));

        components
            .iter()
            .enumerate()
            .map(|(index, component)| {
                if matches!(*component, "" | "." | ".." | "~") {
                    (*component).to_string()
                } else if Some(index) == last_name_index {
                    source_file.format_component(component)
                } else {
                    source_dir.format_component(component)
                }
            })
            .collect::<Vec<_>>()
            .join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_format_component() {
        assert_eq!(
            GtpLangConfigNamingCase::CamelCase.format_component("ShopGoods"),
            "shopGoods"
        );
        assert_eq!(
            GtpLangConfigNamingCase::PascalCase.format_component("shop_goods"),
            "ShopGoods"
        );
        assert_eq!(
            GtpLangConfigNamingCase::SnakeCase.format_component("ShopGoods"),
            "shop_goods"
        );
        assert_eq!(
            GtpLangConfigNamingCase::KebabCase.format_component("ShopGoods"),
            "shop-goods"
        );
    }

    #[test]
    fn test_format_source_file_path() {
        assert_eq!(
            GtpLangConfigNamingCase::format_file_path(
                "../ShopGoods/OrderItem",
                GtpLangConfigNamingCase::SnakeCase,
                GtpLangConfigNamingCase::KebabCase,
            ),
            "../shop_goods/order-item"
        );
        assert_eq!(
            GtpLangConfigNamingCase::format_file_path(
                "~/ShopGoods/OrderItem",
                GtpLangConfigNamingCase::KebabCase,
                GtpLangConfigNamingCase::CamelCase,
            ),
            "~/shop-goods/orderItem"
        );
    }
}
