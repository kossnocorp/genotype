use crate::prelude::internal::*;

impl GtParse for GtAttributeDescriptor {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;

        let mut inner = token.into_semantic_children();
        let token = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::AttributeDescriptor,
            "attribute descriptor inner",
        ))?;

        match token.kind {
            GtTokenKind::AttributeAssignment => Ok(GtAttributeDescriptor::Assignment(
                GtAttributeAssignment::parse(token, context)?,
            )),

            GtTokenKind::AttributeArguments => {
                let arguments = token
                    .into_semantic_children()
                    .map(|token| GtAttributeValue::parse(token, context))
                    .collect::<Result<_, _>>()?;
                Ok(GtAttributeDescriptor::Arguments(arguments))
            }

            GtTokenKind::AttributeProperties => {
                let properties = token
                    .into_semantic_children()
                    .map(|kind| GtAttributeProperty::parse(kind, context))
                    .collect::<Result<_, _>>()?;
                Ok(GtAttributeDescriptor::Properties(properties))
            }

            kind => Err(GtParseError::UnexpectedToken(
                span,
                GtNode::AttributeDescriptor,
                kind,
                "expected attribute descriptor",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_parse_assignment() {
        let mut tokens = GtParser::parse(GtTokenKind::AttributeDescriptor, "= 42").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeDescriptor::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @"
        Assignment(GtAttributeAssignment(
          span: GtSpan(0, 4),
          value: Literal(GtLiteral(
            span: GtSpan(2, 4),
            doc: None,
            attributes: [],
            value: Integer(42),
          )),
        ))
        "
        );
    }

    #[test]
    fn test_parse_arguments() {
        let mut tokens =
            GtParser::parse(GtTokenKind::AttributeDescriptor, r#"("hello", "world")"#).unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeDescriptor::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @r#"
        Arguments([
          Literal(GtLiteral(
            span: GtSpan(1, 8),
            doc: None,
            attributes: [],
            value: String("hello"),
          )),
          Literal(GtLiteral(
            span: GtSpan(10, 17),
            doc: None,
            attributes: [],
            value: String("world"),
          )),
        ])
        "#
        );
    }

    #[test]
    fn test_parse_properties() {
        let mut tokens = GtParser::parse(
            GtTokenKind::AttributeDescriptor,
            r#"(hello = "world", qwe = 123)"#,
        )
        .unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeDescriptor::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @r#"
        Properties([
          GtAttributeProperty(
            span: GtSpan(1, 16),
            name: GtAttributeKey(
              span: GtSpan(1, 6),
              value: "hello",
            ),
            value: Literal(GtLiteral(
              span: GtSpan(9, 16),
              doc: None,
              attributes: [],
              value: String("world"),
            )),
          ),
          GtAttributeProperty(
            span: GtSpan(18, 27),
            name: GtAttributeKey(
              span: GtSpan(18, 21),
              value: "qwe",
            ),
            value: Literal(GtLiteral(
              span: GtSpan(24, 27),
              doc: None,
              attributes: [],
              value: Integer(123),
            )),
          ),
        ])
        "#
        );
    }
}
