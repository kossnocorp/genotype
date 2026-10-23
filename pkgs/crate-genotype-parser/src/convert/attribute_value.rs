use crate::prelude::internal::*;

impl GtParse for GtAttributeValue {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = token.span;

        let mut inner = token.into_semantic_children();
        let token = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::AttributeValue,
            "attribute value inner",
        ))?;

        match token.kind {
            GtTokenKind::Literal => {
                Ok(GtAttributeValue::Literal(GtLiteral::parse(token, context)?))
            }

            GtTokenKind::Name => Ok(GtAttributeValue::Identifier(GtIdentifier::parse(token))),

            kind => Err(GtParseError::UnexpectedToken(
                span,
                GtNode::AttributeValue,
                kind,
                "expected literal or identifier",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_parse_literal() {
        let mut tokens = GtParser::parse(GtTokenKind::AttributeValue, "42").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeValue::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @"
        Literal(GtLiteral(
          span: GtSpan(0, 2),
          doc: None,
          attributes: [],
          value: Integer(42),
        ))
        "
        );
    }

    #[test]
    fn test_parse_identifier() {
        let mut tokens = GtParser::parse(GtTokenKind::AttributeValue, "hello").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeValue::parse(tokens.next().unwrap(), &mut context).unwrap(),
            @r#"Identifier(GtIdentifier(GtSpan(0, 5), "hello"))"#
        );
    }
}
