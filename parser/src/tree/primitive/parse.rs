use pest::iterators::Pair;

use crate::{diagnostic::error::GTParseError, parser::Rule, GTNode};

use super::GTPrimitive;

impl TryFrom<Pair<'_, Rule>> for GTPrimitive {
    type Error = GTParseError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let span = pair.as_span().into();

        match pair.as_str() {
            "boolean" => Ok(GTPrimitive::Boolean(span)),
            "string" => Ok(GTPrimitive::String(span)),
            "int" => Ok(GTPrimitive::Int64(span)),
            "i8" => Ok(GTPrimitive::Int8(span)),
            "i16" => Ok(GTPrimitive::Int16(span)),
            "i32" => Ok(GTPrimitive::Int32(span)),
            "i64" => Ok(GTPrimitive::Int64(span)),
            "i128" => Ok(GTPrimitive::Int128(span)),
            "isize" => Ok(GTPrimitive::IntSize(span)),
            "uint" => Ok(GTPrimitive::IntU32(span)),
            "u8" => Ok(GTPrimitive::IntU8(span)),
            "u16" => Ok(GTPrimitive::IntU16(span)),
            "u32" => Ok(GTPrimitive::IntU32(span)),
            "u64" => Ok(GTPrimitive::IntU64(span)),
            "u128" => Ok(GTPrimitive::IntU128(span)),
            "usize" => Ok(GTPrimitive::IntUSize(span)),
            "float" => Ok(GTPrimitive::Float64(span)),
            "f32" => Ok(GTPrimitive::Float32(span)),
            "f64" => Ok(GTPrimitive::Float64(span)),
            "null" => Ok(GTPrimitive::Null(span)),
            _ => Err(GTParseError::Internal(span, GTNode::Primitive)),
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
            GTParseError::Internal((0, 5).into(), GTNode::Primitive)
        );
    }

    #[test]
    fn test_int_sizes() {
        let mut pairs = GenotypeParser::parse(Rule::primitive, "i8").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::Int8(GTSpan(0, 2))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "i16").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::Int16(GTSpan(0, 3))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "i32").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::Int32(GTSpan(0, 3))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "i64").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::Int64(GTSpan(0, 3))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "i128").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::Int128(GTSpan(0, 4))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "isize").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::IntSize(GTSpan(0, 5))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "u8").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::IntU8(GTSpan(0, 2))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "u16").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::IntU16(GTSpan(0, 3))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "u32").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::IntU32(GTSpan(0, 3))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "u64").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::IntU64(GTSpan(0, 3))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "u128").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::IntU128(GTSpan(0, 4))
        );
        let mut pairs = GenotypeParser::parse(Rule::primitive, "usize").unwrap();
        assert_eq!(
            GTPrimitive::try_from(pairs.next().unwrap()).unwrap(),
            GTPrimitive::IntUSize(GTSpan(0, 5))
        );
    }
}
