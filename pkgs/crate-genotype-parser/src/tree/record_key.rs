use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GtRecordKey {
    String(GtSpan),
    Number(GtSpan),
    Int8(GtSpan),
    Int16(GtSpan),
    Int32(GtSpan),
    Int64(GtSpan),
    Int128(GtSpan),
    IntSize(GtSpan),
    IntU8(GtSpan),
    IntU16(GtSpan),
    IntU32(GtSpan),
    IntU64(GtSpan),
    IntU128(GtSpan),
    IntUSize(GtSpan),
    Float32(GtSpan),
    Float64(GtSpan),
}

impl GtRecordKey {
    pub fn parse(pair: Pair<'_, Rule>) -> GtNodeParseResult<Self> {
        let span = pair.as_span().into();

        match pair.clone().into_inner().as_str() {
            "" | "string" => Ok(GtRecordKey::String(span)),
            "number" => Ok(GtRecordKey::Number(span)),
            "int" => Ok(GtRecordKey::Int64(span)),
            "i8" => Ok(GtRecordKey::Int8(span)),
            "i16" => Ok(GtRecordKey::Int16(span)),
            "i32" => Ok(GtRecordKey::Int32(span)),
            "i64" => Ok(GtRecordKey::Int64(span)),
            "i128" => Ok(GtRecordKey::Int128(span)),
            "isize" => Ok(GtRecordKey::IntSize(span)),
            "uint" => Ok(GtRecordKey::IntU32(span)),
            "u8" => Ok(GtRecordKey::IntU8(span)),
            "u16" => Ok(GtRecordKey::IntU16(span)),
            "u32" => Ok(GtRecordKey::IntU32(span)),
            "u64" => Ok(GtRecordKey::IntU64(span)),
            "u128" => Ok(GtRecordKey::IntU128(span)),
            "usize" => Ok(GtRecordKey::IntUSize(span)),
            "float" => Ok(GtRecordKey::Float64(span)),
            "f32" => Ok(GtRecordKey::Float32(span)),
            "f64" => Ok(GtRecordKey::Float64(span)),
            _ => Err(GtParseError::UnexpectedRule(
                span,
                GtNode::RecordKey,
                pair.as_rule(),
                "expected record key primitive",
            )),
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
            GtRecordKey::String((0, 2).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_string() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[string]").unwrap();
        assert_eq!(
            GtRecordKey::String((0, 8).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_int() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[int]").unwrap();
        assert_eq!(
            GtRecordKey::Int64((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i8]").unwrap();
        assert_eq!(
            GtRecordKey::Int8((0, 4).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i16]").unwrap();
        assert_eq!(
            GtRecordKey::Int16((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i32]").unwrap();
        assert_eq!(
            GtRecordKey::Int32((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i64]").unwrap();
        assert_eq!(
            GtRecordKey::Int64((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[i128]").unwrap();
        assert_eq!(
            GtRecordKey::Int128((0, 6).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[isize]").unwrap();
        assert_eq!(
            GtRecordKey::IntSize((0, 7).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u8]").unwrap();
        assert_eq!(
            GtRecordKey::IntU8((0, 4).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u16]").unwrap();
        assert_eq!(
            GtRecordKey::IntU16((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u32]").unwrap();
        assert_eq!(
            GtRecordKey::IntU32((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u64]").unwrap();
        assert_eq!(
            GtRecordKey::IntU64((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[u128]").unwrap();
        assert_eq!(
            GtRecordKey::IntU128((0, 6).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[usize]").unwrap();
        assert_eq!(
            GtRecordKey::IntUSize((0, 7).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_float() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[float]").unwrap();
        assert_eq!(
            GtRecordKey::Float64((0, 7).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[f32]").unwrap();
        assert_eq!(
            GtRecordKey::Float32((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[f64]").unwrap();
        assert_eq!(
            GtRecordKey::Float64((0, 5).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }

    #[test]
    fn test_parse_number() {
        let mut pairs = GenotypeParser::parse(Rule::record_key, "[number]").unwrap();
        assert_eq!(
            GtRecordKey::Number((0, 8).into()),
            GtRecordKey::parse(pairs.next().unwrap()).unwrap(),
        );
    }
}
