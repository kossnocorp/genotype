use crate::prelude::internal::*;

impl GtParse for GtAttributeValue {
    fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = (pair.as_span().start(), pair.as_span().end()).into();

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::AttributeValue,
            "attribute value inner",
        ))?;

        match pair.as_rule() {
            Rule::literal => Ok(GtAttributeValue::Literal(GtLiteral::parse(pair, context)?)),

            Rule::name => Ok(GtAttributeValue::Identifier(GtIdentifier::parse(pair))),

            rule => Err(GtParseError::UnexpectedRule(
                span,
                GtNode::AttributeValue,
                rule,
                "expected literal or identifier",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use insta::assert_ron_snapshot;
    use pest::Parser;

    #[test]
    fn test_parse_literal() {
        let mut pairs = GenotypeParser::parse(Rule::attribute_value, "42").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeValue::parse(pairs.next().unwrap(), &mut context).unwrap(),
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
        let mut pairs = GenotypeParser::parse(Rule::attribute_value, "hello").unwrap();
        let mut context = GtContext::new("module".into());
        assert_ron_snapshot!(
            GtAttributeValue::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"Identifier(GtIdentifier(GtSpan(0, 5), "hello"))"#
        );
    }
}
