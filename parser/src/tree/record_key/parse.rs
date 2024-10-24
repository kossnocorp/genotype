use pest::iterators::Pair;

use crate::{parser::Rule, GTNode, GTNodeParseResult, GTParseError};

use super::GTRecordKey;

impl GTRecordKey {
    pub fn parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<Self> {
        let span = pair.as_span().into();

        let str = pair
            .into_inner()
            .next()
            .and_then(|pair| Some(pair.as_str()))
            .unwrap_or("string");

        match str {
            "string" => Ok(GTRecordKey::String(span)),
            "int" => Ok(GTRecordKey::Int(span)),
            "float" => Ok(GTRecordKey::Float(span)),
            "boolean" => Ok(GTRecordKey::Boolean(span)),
            _ => Err(GTParseError::UnknownRule(span, GTNode::RecordKey)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_default() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[]").unwrap();
        assert_eq!(
            GTRecordKey::String((0, 2).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_string() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[string]").unwrap();
        assert_eq!(
            GTRecordKey::String((0, 8).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_int() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[int]").unwrap();
        assert_eq!(
            GTRecordKey::Int((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_float() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[float]").unwrap();
        assert_eq!(
            GTRecordKey::Float((0, 7).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_boolean() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[boolean]").unwrap();
        assert_eq!(
            GTRecordKey::Boolean((0, 9).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }
}
