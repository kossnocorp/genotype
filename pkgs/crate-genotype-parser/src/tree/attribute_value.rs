use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GTAttributeValue {
    Literal(#[visit] GTLiteral),
    Identifier(#[visit] GTIdentifier),
}

impl From<GTLiteral> for GTAttributeValue {
    fn from(literal: GTLiteral) -> Self {
        Self::Literal(literal)
    }
}

impl From<GTIdentifier> for GTAttributeValue {
    fn from(identifier: GTIdentifier) -> Self {
        Self::Identifier(identifier)
    }
}

impl GTAttributeValue {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::AttributeValue))?;

        match pair.as_rule() {
            Rule::literal => Ok(GTAttributeValue::Literal(GTLiteral::parse(pair, context)?)),

            Rule::name => Ok(GTAttributeValue::Identifier(pair.into())),

            rule => Err(GTParseError::UnexpectedRule(
                span,
                GTNode::AttributeValue,
                rule,
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
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttributeValue::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @"
        Literal(GTLiteral(
          span: GTSpan(0, 2),
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
        let mut context = GTContext::new("module".into());
        assert_ron_snapshot!(
            GTAttributeValue::parse(pairs.next().unwrap(), &mut context).unwrap(),
            @r#"Identifier(GTIdentifier(GTSpan(0, 5), "hello"))"#
        );
    }
}
