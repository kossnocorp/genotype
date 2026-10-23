use crate::prelude::internal::*;

impl GtParse for GtArray {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;
        let (doc, attributes) = context.take_annotation_or_default();
        let token = token
            .into_semantic_children()
            .next()
            .ok_or(GtParseError::UnexpectedEnd(
                span,
                GtNode::Array,
                "array descriptor",
            ))?;
        let descriptor = GtDescriptor::parse(token, context)?;
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
        let mut tokens = GtParser::parse(GtTokenKind::Array, "[string]").unwrap();
        assert_ron_snapshot!(
            GtArray::parse(tokens.next().unwrap(), &mut GtContext::new("module".into())).unwrap(),
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
        let mut tokens = GtParser::parse(GtTokenKind::LiteralBoolean, "false").unwrap();
        assert_equal!(
            GtArray::parse(tokens.next().unwrap(), &mut GtContext::new("module".into()))
                .unwrap_err(),
            GtParseError::UnexpectedEnd((0, 5).into(), GtNode::Array, "array descriptor")
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
            parse_node!(GtArray, (to_parse_tokens(GtTokenKind::Array, "[string]"), &mut context)),
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
