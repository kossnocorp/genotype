use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtLiteral {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    pub value: GtLiteralValue,
}

impl GtLiteral {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> Result<Self, GtParseError> {
        let span: GtSpan = pair.as_span().into();
        let value = GtLiteralValue::parse(pair, context)?;
        let (doc, attributes) = context.take_annotation_or_default();

        Ok(GtLiteral {
            span,
            doc,
            attributes,
            value,
        })
    }
}

impl Display for GtLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::literal, "420").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtLiteral::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        GtLiteral(
          span: GtSpan(0, 3),
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
            parse_node_err!(GtLiteral, to_parse_args(Rule::object, "{}")),
            @"
        Internal(
            GtSpan(
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
            parse_node!(GtLiteral, (to_parse_rules(Rule::literal, "42"), &mut context)),
            @r#"
        GtLiteral(
          span: GtSpan(0, 2),
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
          value: Integer(42),
        )
        "#
        );
    }
}
