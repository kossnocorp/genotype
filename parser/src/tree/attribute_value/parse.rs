use pest::iterators::Pair;

use crate::*;

use super::GTAttributeValue;

impl GTAttributeValue {
    pub fn parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<Self> {
        let span: GTSpan = pair.as_span().into();

        let mut inner = pair.into_inner();
        let pair = inner
            .next()
            .ok_or_else(|| GTParseError::UnexpectedEnd(span.clone(), GTNode::AttributeValue))?;

        match pair.as_rule() {
            Rule::literal => Ok(GTAttributeValue::Literal(pair.try_into()?)),

            Rule::name => Ok(GTAttributeValue::Identifier(pair.into())),

            _ => Err(GTParseError::UnknownRule(span, GTNode::AttributeValue)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_literal() {
        let mut pairs = GenotypeParser::parse(Rule::attribute_value, "42").unwrap();
        assert_eq!(
            GTAttributeValue::Literal(GTLiteral::Integer((0, 2).into(), 42)),
            GTAttributeValue::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_identifier() {
        let mut pairs = GenotypeParser::parse(Rule::attribute_value, "hello").unwrap();
        assert_eq!(
            GTAttributeValue::Identifier(GTIdentifier::new((0, 5).into(), "hello".into())),
            GTAttributeValue::parse(pairs.next().unwrap()).unwrap(),
        );
    }
}
