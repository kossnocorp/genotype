use crate::prelude::internal::*;

impl TSConvert<TSBranded> for GTBranded {
    fn convert(&self, context: &mut TSConvertContext) -> TSBranded {
        let doc = context.consume_doc();
        let name = self.name.convert(context);
        let primitive = self.primitive.convert(context);

        TSBranded {
            doc,
            name,
            primitive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            convert_to_ts(
                GTBranded {
                    span: (0, 0).into(),
                    id: GTDefinitionId("module".into(), "UserId".into()),
                    name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                    primitive: GtFactory::primitive_string().into(),
                }
            ),
            @r#"
        TSBranded(
          doc: None,
          name: TSIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }

    #[test]
    fn test_doc() {
        assert_ron_snapshot!(
            convert_to_ts_with(
                GTBranded {
                    span: (0, 0).into(),
                    id: GTDefinitionId("module".into(), "UserId".into()),
                    name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                    primitive: GtFactory::primitive_string().into(),
                },
                |context| {
                    context.provide_doc(Some("This is a user ID.".into()));
                },
            ),
            @r#"
        TSBranded(
          doc: Some(TSDoc("This is a user ID.")),
          name: TSIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }
}
