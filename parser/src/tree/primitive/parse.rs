use pest::iterators::Pair;

use crate::{diagnostic::error::GTNodeParseError, parser::Rule, GTNode};

use super::GTPrimitive;

impl TryFrom<Pair<'_, Rule>> for GTPrimitive {
    type Error = GTNodeParseError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let span = pair.as_span().into();

        match pair.as_str() {
            "boolean" => Ok(GTPrimitive::Boolean(span)),

            "string" => Ok(GTPrimitive::String(span)),

            "int" => Ok(GTPrimitive::Int(span)),

            "float" => Ok(GTPrimitive::Float(span)),

            "null" => Ok(GTPrimitive::Null(span)),

            _ => Err(GTNodeParseError::Internal(span, GTNode::Primitive)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;

    #[test]
    fn test_from_pair() {
        let mut pairs = GenotypeParser::parse(Rule::primitive, "boolean").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::Boolean(GTSpan(0, 7))
        );
    }

    #[test]
    fn test_error() {
        let mut pairs = GenotypeParser::parse(Rule::literal_boolean, "false").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap_err(),
            GTNodeParseError::Internal((0, 5).into(), GTNode::Primitive)
        );
    }
}
