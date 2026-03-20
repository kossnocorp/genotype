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
            convert_node(
                Gt::branded(
                    "UserId",
                    Gt::primitive_string(),
                )
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
        let mut context = TSConvertContext::default();
        context.provide_doc(Some("This is a user ID.".into()));
        assert_ron_snapshot!(
            convert_node_with(
                Gt::branded(
                    "UserId",
                    Gt::primitive_string(),
                ),
                &mut context
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
