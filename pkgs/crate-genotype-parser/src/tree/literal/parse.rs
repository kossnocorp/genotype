use crate::prelude::internal::*;

impl GTLiteral {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span: GTSpan = pair.as_span().into();
        let value = GTLiteralValue::parse(pair, context)?;
        let (doc, attributes) = context.take_annotation_or_default();

        Ok(GTLiteral {
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
        let mut pairs = GenotypeParser::parse(Rule::literal, "420").unwrap();
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTLiteral::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        GTLiteral(
          span: GTSpan(0, 3),
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
            parse_node_err!(GTLiteral, to_parse_args(Rule::object, "{}")),
            @"
        Internal(
            GTSpan(
                0,
                2,
            ),
            Literal,
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
            parse_node!(GTLiteral, (to_parse_rules(Rule::literal, "42"), &mut context)),
            @r#"
        GTLiteral(
          span: GTSpan(0, 2),
          doc: Some(GTDoc(GTSpan(0, 0), "Hello, world!")),
          attributes: [
            GTAttribute(
              span: GTSpan(0, 2),
              name: GTAttributeName(
                span: GTSpan(0, 0),
                value: "example",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(0, 0),
                value: Literal(GTLiteral(
                  span: GTSpan(0, 0),
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
