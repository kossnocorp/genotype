use crate::prelude::internal::*;

impl PYConvert<PYNewtype> for GTBranded {
    fn convert(&self, context: &mut PYConvertContext) -> PYNewtype {
        let doc = context.consume_doc();
        let name = self.name.convert(context);
        let primitive = self.primitive.convert(context);

        PYNewtype {
            doc,
            name,
            primitive,
        }
        .resolve(context)
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
                Gt::branded("UserId", Gt::primitive_string())
            ),
            @r#"
        PYNewtype(
          doc: None,
          name: PYIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            convert_node_with(
                Gt::branded("UserId", Gt::primitive_string()),
                &mut context
            ),
            @r#"
        PYNewtype(
          doc: None,
          name: PYIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("NewType")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = PYConvertContext::default();
        context.provide_doc(Some(PYDoc("Hello, world!".into())));
        assert_ron_snapshot!(
            convert_node_with(
                Gt::branded("UserId", Gt::primitive_string()),
                &mut context
            ),
            @r#"
        PYNewtype(
          doc: Some(PYDoc("Hello, world!")),
          name: PYIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }
}
