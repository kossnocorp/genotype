use crate::prelude::internal::*;

impl GTAny {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();

        Ok(GTAny {
            span,
            doc,
            attributes,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test::*;
    use crate::*;

    #[test]
    fn test_from_pair() {
        assert_ron_snapshot!(
            parse_node!(GTAny, to_parse_args(Rule::any, "any")),
            @"
        GTAny(
          span: GTSpan(0, 3),
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
            parse_node!(GTAny, (to_parse_rules(Rule::any, "any"), &mut context)),
            @r#"
        GTAny(
          span: GTSpan(0, 3),
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
        )
        "#
        );
    }
}
