use crate::prelude::internal::*;

impl GtParse for GtAny {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span = token.span;
        let (doc, attributes) = context.take_annotation_or_default();

        Ok(GtAny {
            span,
            doc,
            attributes,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use crate::test::*;

    #[test]
    fn test_from_token() {
        assert_ron_snapshot!(
            parse_node!(GtAny, to_parse_args(GtTokenKind::Any, "any")),
            @"
        GtAny(
          span: GtSpan(0, 3),
          doc: None,
          attributes: [],
        )
        "
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
            parse_node!(GtAny, (to_parse_tokens(GtTokenKind::Any, "any"), &mut context)),
            @r#"
        GtAny(
          span: GtSpan(0, 3),
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
