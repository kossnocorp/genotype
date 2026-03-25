use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTPrimitive {
    pub span: GTSpan,
    pub kind: GTPrimitiveKind,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
}

impl Display for GTPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Into<GTDescriptor> for GTPrimitive {
    fn into(self) -> GTDescriptor {
        GTDescriptor::Primitive(self)
    }
}

impl GTPrimitive {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span = pair.as_span().into();
        let kind = GTPrimitiveKind::parse(pair, context)?;
        let (doc, attributes) = context.take_annotation_or_default();

        Ok(GTPrimitive {
            span,
            doc,
            attributes,
            kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GTPrimitive, to_parse_args(Rule::primitive, "boolean")),
            @"
        GTPrimitive(
          span: GTSpan(0, 7),
          kind: Boolean,
          doc: None,
          attributes: [],
        )
        "
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitive, to_parse_args(Rule::primitive, "string")),
            @"
        GTPrimitive(
          span: GTSpan(0, 6),
          kind: String,
          doc: None,
          attributes: [],
        )
        "
        );

        assert_ron_snapshot!(
            parse_node!(GTPrimitive, to_parse_args(Rule::primitive, "number")),
            @"
        GTPrimitive(
          span: GTSpan(0, 6),
          kind: Number,
          doc: None,
          attributes: [],
        )
        "
        );
    }

    #[test]
    fn test_error() {
        assert_debug_snapshot!(
            parse_node_err!(GTPrimitive, to_parse_args(Rule::literal_boolean, "false")),
            @"
        Internal(
            GTSpan(
                0,
                5,
            ),
            Primitive,
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
            parse_node!(GTPrimitive, (to_parse_rules(Rule::primitive, "string"), &mut context)),
            @r#"
        GTPrimitive(
          span: GTSpan(0, 6),
          kind: String,
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
