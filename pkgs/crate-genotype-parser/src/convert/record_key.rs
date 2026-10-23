use crate::prelude::internal::*;

impl GtParse for GtRecordKey {
    fn parse(token: GtToken<'_>, context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = token.span;
        let mut inner = token.clone().into_semantic_children();

        if let Some(inner) = inner.next()
            && inner.kind == GtTokenKind::Name
        {
            let identifier = GtIdentifier::parse(inner);
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

        match token
            .clone()
            .into_semantic_children()
            .next()
            .map_or("", |token| token.text)
        {
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
            _ => Err(GtParseError::UnexpectedToken(
                span,
                GtNode::RecordKey,
                token.kind,
                "expected record key primitive",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::internal::*;
    use crate::test::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_default() {
        assert_eq!(
            GtRecordKey::String((0, 2).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[]"))
        );
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            GtRecordKey::String((0, 8).into()),
            parse_node!(
                GtRecordKey,
                to_parse_args(GtTokenKind::RecordKey, "[string]")
            )
        );
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(
            GtRecordKey::Boolean((0, 9).into()),
            parse_node!(
                GtRecordKey,
                to_parse_args(GtTokenKind::RecordKey, "[boolean]")
            )
        );
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(
            GtRecordKey::Int64((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[int]"))
        );
        assert_eq!(
            GtRecordKey::Int8((0, 4).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[i8]"))
        );
        assert_eq!(
            GtRecordKey::Int16((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[i16]"))
        );
        assert_eq!(
            GtRecordKey::Int32((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[i32]"))
        );
        assert_eq!(
            GtRecordKey::Int64((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[i64]"))
        );
        assert_eq!(
            GtRecordKey::Int128((0, 6).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[i128]"))
        );
        assert_eq!(
            GtRecordKey::IntSize((0, 7).into()),
            parse_node!(
                GtRecordKey,
                to_parse_args(GtTokenKind::RecordKey, "[isize]")
            )
        );
        assert_eq!(
            GtRecordKey::IntU8((0, 4).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[u8]"))
        );
        assert_eq!(
            GtRecordKey::IntU16((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[u16]"))
        );
        assert_eq!(
            GtRecordKey::IntU32((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[u32]"))
        );
        assert_eq!(
            GtRecordKey::IntU64((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[u64]"))
        );
        assert_eq!(
            GtRecordKey::IntU128((0, 6).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[u128]"))
        );
        assert_eq!(
            GtRecordKey::IntUSize((0, 7).into()),
            parse_node!(
                GtRecordKey,
                to_parse_args(GtTokenKind::RecordKey, "[usize]")
            )
        );
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(
            GtRecordKey::Float64((0, 7).into()),
            parse_node!(
                GtRecordKey,
                to_parse_args(GtTokenKind::RecordKey, "[float]")
            )
        );
        assert_eq!(
            GtRecordKey::Float32((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[f32]"))
        );
        assert_eq!(
            GtRecordKey::Float64((0, 5).into()),
            parse_node!(GtRecordKey, to_parse_args(GtTokenKind::RecordKey, "[f64]"))
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(
            GtRecordKey::Number((0, 8).into()),
            parse_node!(
                GtRecordKey,
                to_parse_args(GtTokenKind::RecordKey, "[number]")
            )
        );
    }
}
