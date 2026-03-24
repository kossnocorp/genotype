use crate::prelude::internal::*;

impl GTArray {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();
        let pair = pair
            .into_inner()
            .next()
            .ok_or_else(|| GTParseError::Internal(span.clone(), GTNode::Array))?;
        let descriptor = GTDescriptor::parse(pair, context)?;
        Ok(GTArray {
            span,
            doc,
            attributes,
            descriptor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::array, "[string]").unwrap();
        assert_ron_snapshot!(
            GTArray::parse(pairs.next().unwrap(), &mut GTContext::new("module".into())).unwrap(),
            @"
        GTArray(
          span: GTSpan(0, 8),
          doc: None,
          attributes: [],
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(1, 7),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::literal_boolean, "false").unwrap();
        assert_equal!(
            GTArray::parse(pairs.next().unwrap(), &mut GTContext::new("module".into()))
                .unwrap_err(),
            GTParseError::Internal((0, 5).into(), GTNode::Array)
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
            parse_node!(GTArray, (to_parse_rules(Rule::array, "[string]"), &mut context)),
            @r#"
        GTArray(
          span: GTSpan(0, 8),
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
          descriptor: Primitive(GTPrimitive(
            span: GTSpan(1, 7),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "#
        );
    }
}
