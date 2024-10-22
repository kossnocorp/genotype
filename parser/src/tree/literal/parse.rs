use pest::iterators::Pair;

use crate::{diagnostic::error::GTParseError, parser::Rule, GTNode, GTSpan};

use super::GTLiteral;

impl TryFrom<Pair<'_, Rule>> for GTLiteral {
    type Error = GTParseError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let span: GTSpan = pair.as_span().into();
        let else_err = || GTParseError::Internal(span.clone(), GTNode::Literal);
        let pair = pair.into_inner().next().ok_or_else(else_err)?;

        match pair.as_rule() {
            Rule::literal_string => {
                let pair = pair.into_inner().next().ok_or_else(else_err)?;
                Ok(GTLiteral::String(span, pair.as_str().into()))
            }

            Rule::literal_integer => {
                let value = pair
                    .as_str()
                    .replace("_", "")
                    .parse()
                    .map_err(|_| else_err())?;
                Ok(GTLiteral::Integer(span, value))
            }

            Rule::literal_float => {
                let value = pair
                    .as_str()
                    .replace("_", "")
                    .parse()
                    .map_err(|_| else_err())?;
                Ok(GTLiteral::Float(span, value))
            }

            Rule::literal_boolean => {
                let value = pair.as_str().parse().map_err(|_| else_err())?;
                Ok(GTLiteral::Boolean(span, value))
            }

            _ => Err(else_err()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse() {
        let mut pairs = GenotypeParser::parse(Rule::literal, "420").unwrap();
        assert_eq!(
            GTLiteral::Integer((0, 3).into(), 420),
            pairs.next().unwrap().try_into().unwrap()
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::object, "{}").unwrap();
        assert_eq!(
            GTParseError::Internal((0, 2).into(), GTNode::Literal),
            TryInto::<GTLiteral>::try_into(pairs.next().unwrap()).unwrap_err(),
        );
    }
}
