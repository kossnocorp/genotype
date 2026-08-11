use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GtRecordKey {
    Reference(#[visit] GtReference),
    Boolean(GtSpan),
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
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = pair.as_span().into();
        let mut inner = pair.clone().into_inner();

        if let Some(inner) = inner.next()
            && inner.as_rule() == Rule::name
        {
            let identifier: GtIdentifier = inner.into();
            let reference_span = identifier.0;
            context.resolve.references.insert(identifier.clone());
            context.resolve_reference_identifier_as_generic_parameter(&identifier);

            return Ok(GtRecordKey::Reference(GtReference {
                span: reference_span,
                doc: None,
                attributes: vec![],
                id: GtReferenceId(context.module_id.clone(), reference_span),
                identifier,
                arguments: vec![],
            }));
        }

        match pair.clone().into_inner().as_str() {
            "" | "string" => Ok(GtRecordKey::String(span)),
            "boolean" => Ok(GtRecordKey::Boolean(span)),
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
    use crate::test::*;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_default() {
        assert_eq!(
            GtRecordKey::String((0, 2).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[]"))
        );
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            GtRecordKey::String((0, 8).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[string]"))
        );
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(
            GtRecordKey::Boolean((0, 9).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[boolean]"))
        );
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(
            GtRecordKey::Int64((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[int]"))
        );
        assert_eq!(
            GtRecordKey::Int8((0, 4).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[i8]"))
        );
        assert_eq!(
            GtRecordKey::Int16((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[i16]"))
        );
        assert_eq!(
            GtRecordKey::Int32((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[i32]"))
        );
        assert_eq!(
            GtRecordKey::Int64((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[i64]"))
        );
        assert_eq!(
            GtRecordKey::Int128((0, 6).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[i128]"))
        );
        assert_eq!(
            GtRecordKey::IntSize((0, 7).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[isize]"))
        );
        assert_eq!(
            GtRecordKey::IntU8((0, 4).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[u8]"))
        );
        assert_eq!(
            GtRecordKey::IntU16((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[u16]"))
        );
        assert_eq!(
            GtRecordKey::IntU32((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[u32]"))
        );
        assert_eq!(
            GtRecordKey::IntU64((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[u64]"))
        );
        assert_eq!(
            GtRecordKey::IntU128((0, 6).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[u128]"))
        );
        assert_eq!(
            GtRecordKey::IntUSize((0, 7).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[usize]"))
        );
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(
            GtRecordKey::Float64((0, 7).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[float]"))
        );
        assert_eq!(
            GtRecordKey::Float32((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[f32]"))
        );
        assert_eq!(
            GtRecordKey::Float64((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[f64]"))
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(
            GtRecordKey::Number((0, 8).into()),
            parse_node!(GtRecordKey, to_parse_args(Rule::record_key, "[number]"))
        );
    }
}
