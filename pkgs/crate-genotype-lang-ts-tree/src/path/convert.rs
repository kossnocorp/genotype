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
            convert_node(GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap()),
            @r#"TsPath("./path/to/module")"#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut resolve = TsConvertResolve::new();
        resolve.paths.insert(
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap(),
            GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module/index").unwrap(),
        );
        let mut context = TsConvertContext::new(resolve, &Default::default());
        assert_ron_snapshot!(
            convert_node_with(
                GtPath::parse((0, 0).into(), &"module".into(), "./path/to/module").unwrap(),
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
            GtPath::parse((0, 0).into(), &"module".into(), "./ShopGoods/OrderItem")
                .unwrap()
                .convert(&mut context),
            @r#"TsPath("./shop-goods/orderItem")"#
        );
    }
}
