use crate::prelude::internal::*;

impl TsConvert<TsBranded> for GtBranded {
    fn convert(&self, context: &mut TsConvertContext) -> TsBranded {
        let doc = context.consume_doc();
        let name = self.name.convert(context);
        let primitive = self.primitive.convert(context);

        TsBranded {
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
        TsBranded(
          doc: None,
          name: TsIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }

    #[test]
    fn test_doc() {
        let mut context = TsConvertContext::default();
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
        TsBranded(
          doc: Some(TsDoc("This is a user ID.")),
          name: TsIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }
}
