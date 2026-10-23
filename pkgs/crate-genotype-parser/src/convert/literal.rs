use crate::prelude::internal::*;

impl GtParse for GtLiteral {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = token.span;
        let value = GtLiteralValue::parse(token, context)?;
        let (doc, attributes) = context.take_annotation_or_default();

        Ok(GtLiteral {
            span,
            doc,
            attributes,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        let mut tokens = GtParser::parse(GtTokenKind::Literal, "420").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtLiteral::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @"
        GtLiteral(
          span: GtSpan(0, 3),
          doc: None,
          attributes: [],
          value: Integer(420),
        )
        "
        );
    }

    #[test]
    fn test_error() {
        assert_debug_snapshot!(
            parse_node_err!(GtLiteral, to_parse_args(GtTokenKind::Object, "{}")),
            @r#"
        UnexpectedEnd(
            GtSpan(
                0,
                2,
            ),
            Literal,
            "literal value",
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
            parse_node!(GtLiteral, (to_parse_tokens(GtTokenKind::Literal, "42"), &mut context)),
            @r#"
        GtLiteral(
          span: GtSpan(0, 2),
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
          value: Integer(42),
        )
        "#
        );
    }
}
