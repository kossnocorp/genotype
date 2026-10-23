use crate::prelude::internal::*;

impl GtParse for GtPrimitive {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = token.span;
        let kind = GtPrimitiveKind::parse(token, context)?;
        let (doc, attributes) = context.take_annotation_or_default();

        Ok(GtPrimitive {
            span,
            doc,
            attributes,
            kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GtPrimitive, to_parse_args(GtTokenKind::Primitive, "boolean")),
            @"
        GtPrimitive(
          span: GtSpan(0, 7),
          kind: Boolean,
          doc: None,
          attributes: [],
        )
        "
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitive, to_parse_args(GtTokenKind::Primitive, "string")),
            @"
        GtPrimitive(
          span: GtSpan(0, 6),
          kind: String,
          doc: None,
          attributes: [],
        )
        "
        );

        assert_ron_snapshot!(
            parse_node!(GtPrimitive, to_parse_args(GtTokenKind::Primitive, "number")),
            @"
        GtPrimitive(
          span: GtSpan(0, 6),
          kind: Number,
          doc: None,
          attributes: [],
        )
        "
        );
    }

    #[test]
    fn test_error() {
        assert_debug_snapshot!(
            parse_node_err!(GtPrimitive, to_parse_args(GtTokenKind::LiteralBoolean, "false")),
            @r#"
        UnexpectedToken(
            GtSpan(
                0,
                5,
            ),
            Primitive,
            literal_boolean,
            "expected primitive kind",
        )
        "#
        );
    }

    #[test]
    fn test_annotation() {
        let mut context = Gt::context();
        context.provide_annotation((
            Gt::some_doc("Hello, world!"),
            vec![Gt::attribute(
                "example",
                Gt::attribute_assignment(Gt::literal_string("value")),
            )],
        ));
        assert_ron_snapshot!(
            parse_node!(GtPrimitive, (to_parse_tokens(GtTokenKind::Primitive, "string"), &mut context)),
            @r#"
        GtPrimitive(
          span: GtSpan(0, 6),
          kind: String,
          doc: Some(GtDoc(GtSpan(0, 0), "Hello, world!")),
          attributes: [
            GtAttribute(
              span: GtSpan(0, 2),
              name: GtAttributeName(
                span: GtSpan(0, 0),
                value: "example",
              ),
              descriptor: Some(Assignment(GtAttributeAssignment(
                span: GtSpan(0, 0),
                value: Literal(GtLiteral(
                  span: GtSpan(0, 0),
                  doc: None,
                  attributes: [],
                  value: String("value"),
                )),
              ))),
            ),
          ],
        )
        "#
        );
    }
}
