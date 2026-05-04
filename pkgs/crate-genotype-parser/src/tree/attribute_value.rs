use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GtAttributeValue {
    Literal(#[visit] GtLiteral),
    Identifier(#[visit] GtIdentifier),
}

impl From<GtLiteral> for GtAttributeValue {
    fn from(literal: GtLiteral) -> Self {
        Self::Literal(literal)
    }
}

impl From<GtIdentifier> for GtAttributeValue {
    fn from(identifier: GtIdentifier) -> Self {
        Self::Identifier(identifier)
    }
}

impl GtAttributeValue {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span: GtSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner.next().ok_or(GtParseError::UnexpectedEnd(
            span,
            GtNode::AttributeValue,
            "attribute value inner",
        ))?;

        match pair.as_rule() {
            Rule::literal => Ok(GtAttributeValue::Literal(GtLiteral::parse(pair, context)?)),

            Rule::name => Ok(GtAttributeValue::Identifier(pair.into())),

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
    use crate::*;
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
