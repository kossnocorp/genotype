use crate::prelude::internal::*;

impl TsConvert<TsPath> for GtPath {
    fn convert(&self, context: &mut TsConvertContext) -> TsPath {
        TsPath(context.resolve_path(self).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert_base() {
        assert_ron_snapshot!(
            convert_node(Gt::path_anon("./path/to/module")),
            @r#"TsPath("./path/to/module")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TsConvertResolve::new();
        resolve.paths.insert(
            Gt::path_anon("./path/to/module"),
            Gt::path_anon("./path/to/module/index"),
        );
        let mut context = TsConvertContext::new(resolve, &Default::default());
        assert_ron_snapshot!(
            convert_node_with(
                Gt::path_anon("./path/to/module"),
                &mut context,
            ),
            @r#"TsPath("./path/to/module/index")"#
        );
    }

    #[test]
    fn test_convert_naming() {
        let config = TsConfig {
            lang: TsConfigLang {
                naming: TsConfigNaming {
                    source_file: GtpLangConfigNamingCase::CamelCase,
                    source_dir: Some(GtpLangConfigNamingCase::KebabCase),
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let mut context = TsConvertContext::new(Default::default(), &config);

        assert_ron_snapshot!(
            Gt::path_anon("./ShopGoods/OrderItem")
                .convert(&mut context),
            @r#"TsPath("./shop-goods/orderItem")"#
        );
    }
}
