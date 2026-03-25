use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTTuple {
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    #[visit]
    pub descriptors: Vec<GTDescriptor>,
}

impl GTTuple {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> Result<Self, GTParseError> {
        let span = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();

        let mut tuple = GTTuple {
            span,
            doc,
            attributes,
            descriptors: vec![],
        };

        for pair in pair.into_inner() {
            let descriptor = GTDescriptor::parse(pair, context)?;
            tuple.descriptors.push(descriptor);
        }

        Ok(tuple)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GTTuple, to_parse_args(Rule::tuple, "(string, int)")),
            @"
        GTTuple(
          span: GTSpan(0, 13),
          doc: None,
          attributes: [],
          descriptors: [
            Primitive(GTPrimitive(
              span: GTSpan(1, 7),
              kind: String,
              doc: None,
              attributes: [],
            )),
            Primitive(GTPrimitive(
              span: GTSpan(9, 12),
              kind: Int64,
              doc: None,
              attributes: [],
            )),
          ],
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
            parse_node!(GTTuple, (to_parse_rules(Rule::tuple, "(string)"), &mut context)),
            @r#"
        GTTuple(
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
          descriptors: [
            Primitive(GTPrimitive(
              span: GTSpan(1, 7),
              kind: String,
              doc: None,
              attributes: [],
            )),
          ],
        )
        "#
        );
    }
}
