use pest::iterators::Pair;

use crate::{parser::Rule, GTNode, GTNodeParseResult, GTParseError};

use super::GTRecordKey;

impl GTRecordKey {
    pub fn parse(pair: Pair<'_, Rule>) -> GTNodeParseResult<Self> {
        let span = pair.as_span().into();

        match pair.into_inner().as_str() {
            "" | "string" => Ok(GTRecordKey::String(span)),
            "int" => Ok(GTRecordKey::Int32(span)),
            "i8" => Ok(GTRecordKey::Int8(span)),
            "i16" => Ok(GTRecordKey::Int16(span)),
            "i32" => Ok(GTRecordKey::Int32(span)),
            "i64" => Ok(GTRecordKey::Int64(span)),
            "i128" => Ok(GTRecordKey::Int128(span)),
            "isize" => Ok(GTRecordKey::IntSize(span)),
            "uint" => Ok(GTRecordKey::IntU32(span)),
            "u8" => Ok(GTRecordKey::IntU8(span)),
            "u16" => Ok(GTRecordKey::IntU16(span)),
            "u32" => Ok(GTRecordKey::IntU32(span)),
            "u64" => Ok(GTRecordKey::IntU64(span)),
            "u128" => Ok(GTRecordKey::IntU128(span)),
            "usize" => Ok(GTRecordKey::IntUSize(span)),
            "float" => Ok(GTRecordKey::Float64(span)),
            "f32" => Ok(GTRecordKey::Float32(span)),
            "f64" => Ok(GTRecordKey::Float64(span)),
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
            GTRecordKey::Int32((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i8]").unwrap();
        assert_eq!(
            GTRecordKey::Int8((0, 4).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i16]").unwrap();
        assert_eq!(
            GTRecordKey::Int16((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i32]").unwrap();
        assert_eq!(
            GTRecordKey::Int32((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i64]").unwrap();
        assert_eq!(
            GTRecordKey::Int64((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i128]").unwrap();
        assert_eq!(
            GTRecordKey::Int128((0, 6).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[isize]").unwrap();
        assert_eq!(
            GTRecordKey::IntSize((0, 7).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u8]").unwrap();
        assert_eq!(
            GTRecordKey::IntU8((0, 4).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u16]").unwrap();
        assert_eq!(
            GTRecordKey::IntU16((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u32]").unwrap();
        assert_eq!(
            GTRecordKey::IntU32((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u64]").unwrap();
        assert_eq!(
            GTRecordKey::IntU64((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u128]").unwrap();
        assert_eq!(
            GTRecordKey::IntU128((0, 6).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[usize]").unwrap();
        assert_eq!(
            GTRecordKey::IntUSize((0, 7).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_float() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[float]").unwrap();
        assert_eq!(
            GTRecordKey::Float64((0, 7).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[f32]").unwrap();
        assert_eq!(
            GTRecordKey::Float32((0, 5).into()),
            GTRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[f64]").unwrap();
        assert_eq!(
            GTRecordKey::Float64((0, 5).into()),
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
