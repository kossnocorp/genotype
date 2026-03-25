use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtPrimitive {
    pub span: GtSpan,
    pub kind: GtPrimitiveKind,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
}

impl Display for GtPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Into<GtDescriptor> for GtPrimitive {
    fn into(self) -> GtDescriptor {
        GtDescriptor::Primitive(self)
    }
}

impl GtPrimitive {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = pair.as_span().into();
        let kind = GtPrimitiveKind::parse(pair, context)?;
        let (doc, attributes) = context.take_annotation_or_default();

        Ok(GtPrimitive {
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
            parse_node!(GtPrimitive, to_parse_args(Rule::primitive, "boolean")),
            @"
        GtPrimitive(
          span: GtSpan(0, 7),
          kind: Boolean,
          doc: None,
          attributes: [],
        )
        "
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitive, to_parse_args(Rule::primitive, "string")),
            @"
        GtPrimitive(
          span: GtSpan(0, 6),
          kind: String,
          doc: None,
          attributes: [],
        )
        "
        );

        assert_ron_snapshot!(
            parse_node!(GtPrimitive, to_parse_args(Rule::primitive, "number")),
            @"
        GtPrimitive(
          span: GtSpan(0, 6),
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
            parse_node_err!(GtPrimitive, to_parse_args(Rule::literal_boolean, "false")),
            @"
        Internal(
            GtSpan(
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
            parse_node!(GtPrimitive, (to_parse_rules(Rule::primitive, "string"), &mut context)),
            @r#"
        GtPrimitive(
          span: GtSpan(0, 6),
          kind: String,
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
