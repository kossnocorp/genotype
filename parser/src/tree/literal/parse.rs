use crate::prelude::internal::*;

impl GTLiteral {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span: GTSpan = pair.as_span().into();
        let value = GTLiteralValue::parse(pair, context)?;

        let annotation = context.take_annotation();
        let (doc, attributes) = if let Some(annotation) = annotation {
            (annotation.doc, annotation.attributes)
        } else {
            (None, vec![])
        };

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
    use crate::*;
    use insta::assert_ron_snapshot;
    use pest::Parser;
    use pretty_assertions::assert_eq;

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
        let mut pairs = GenotypeParser::parse(Rule::object, "{}").unwrap();
        let mut context = GTContext::new("module".into());
        assert_eq!(
            GTLiteral::parse(pairs.next().unwrap(), &mut context).unwrap_err(),
            GTParseError::Internal((0, 2).into(), GTNode::Literal)
        );
    }

    #[test]
    fn test_annotation() {
        let mut pairs = GenotypeParser::parse(Rule::literal, "420").unwrap();
        let mut context = GTContext::new("module".into());
        context.provide_annotation(GTContextAnnotation {
            doc: Some(GTDoc((0, 0).into(), "Hello, world".into())),
            attributes: vec![GTAttribute {
                span: (0, 2).into(),
                name: GTAttributeName {
                    span: (0, 2).into(),
                    value: "example".into(),
                },
                descriptor: Some(GTAttributeDescriptor::Assignment(GTAttributeAssignment {
                    span: (0, 2).into(),
                    value: GTAttributeValue::Literal(GTLiteral {
                        span: (0, 2).into(),
                        doc: None,
                        attributes: Vec::new(),
                        value: GTLiteralValue::String("value".into()),
                    }),
                })),
            }],
        });
        assert_ron_snapshot!(
            GTLiteral::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"
        GTLiteral(
          span: GTSpan(0, 3),
          doc: Some(GTDoc(GTSpan(0, 0), "Hello, world")),
          attributes: [
            GTAttribute(
              span: GTSpan(0, 2),
              name: GTAttributeName(
                span: GTSpan(0, 2),
                value: "example",
              ),
              descriptor: Some(Assignment(GTAttributeAssignment(
                span: GTSpan(0, 2),
                value: Literal(GTLiteral(
                  span: GTSpan(0, 2),
                  doc: None,
                  attributes: [],
                  value: String("value"),
                )),
              ))),
            ),
          ],
          value: Integer(420),
        )
        "#
        );
    }
}
