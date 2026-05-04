use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtArray {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub descriptor: GtDescriptor,
}

impl GtArray {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();
        let (doc, attributes) = context.take_annotation_or_default();
        let pair = pair
            .into_inner()
            .next()
            .ok_or(GtParseError::InternalLegacy(span, GtNode::Array))?;
        let descriptor = GtDescriptor::parse(pair, context)?;
        Ok(GtArray {
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
            GtArray::parse(pairs.next().unwrap(), &mut GtContext::new("module".into())).unwrap(),
            @"
        GtArray(
          span: GtSpan(0, 8),
          doc: None,
          attributes: [],
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(1, 7),
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
            GtArray::parse(pairs.next().unwrap(), &mut GtContext::new("module".into()))
                .unwrap_err(),
            GtParseError::InternalLegacy((0, 5).into(), GtNode::Array)
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
            parse_node!(GtArray, (to_parse_rules(Rule::array, "[string]"), &mut context)),
            @r#"
        GtArray(
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
          descriptor: Primitive(GtPrimitive(
            span: GtSpan(1, 7),
            kind: String,
            doc: None,
            attributes: [],
          )),
        )
        "#
        );
    }
}
