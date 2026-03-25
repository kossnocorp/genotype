use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtTuple {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub descriptors: Vec<GtDescriptor>,
}

impl GtTuple {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();

        let mut tuple = GtTuple {
            span,
            doc,
            attributes,
            descriptors: vec![],
        };

        for pair in pair.into_inner() {
            let descriptor = GtDescriptor::parse(pair, context)?;
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
            parse_node!(GtTuple, to_parse_args(Rule::tuple, "(string, int)")),
            @"
        GtTuple(
          span: GtSpan(0, 13),
          doc: None,
          attributes: [],
          descriptors: [
            Primitive(GtPrimitive(
              span: GtSpan(1, 7),
              kind: String,
              doc: None,
              attributes: [],
            )),
            Primitive(GtPrimitive(
              span: GtSpan(9, 12),
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
            parse_node!(GtTuple, (to_parse_rules(Rule::tuple, "(string)"), &mut context)),
            @r#"
        GtTuple(
          span: GtSpan(0, 8),
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
          descriptors: [
            Primitive(GtPrimitive(
              span: GtSpan(1, 7),
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
