use crate::prelude::internal::*;

impl GtParse for GtPrimitiveKind {
    fn parse(token: GtToken<'_>, _context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = token.span;
        match token.as_str() {
            "boolean" => Ok(GtPrimitiveKind::Boolean),
            "string" => Ok(GtPrimitiveKind::String),
            "number" => Ok(GtPrimitiveKind::Number),
            "int" => Ok(GtPrimitiveKind::Int64),
            "i8" => Ok(GtPrimitiveKind::Int8),
            "i16" => Ok(GtPrimitiveKind::Int16),
            "i32" => Ok(GtPrimitiveKind::Int32),
            "i64" => Ok(GtPrimitiveKind::Int64),
            "i128" => Ok(GtPrimitiveKind::Int128),
            "isize" => Ok(GtPrimitiveKind::IntSize),
            "uint" => Ok(GtPrimitiveKind::IntU32),
            "u8" => Ok(GtPrimitiveKind::IntU8),
            "u16" => Ok(GtPrimitiveKind::IntU16),
            "u32" => Ok(GtPrimitiveKind::IntU32),
            "u64" => Ok(GtPrimitiveKind::IntU64),
            "u128" => Ok(GtPrimitiveKind::IntU128),
            "usize" => Ok(GtPrimitiveKind::IntUSize),
            "float" => Ok(GtPrimitiveKind::Float64),
            "f32" => Ok(GtPrimitiveKind::Float32),
            "f64" => Ok(GtPrimitiveKind::Float64),
            _ => Err(GtParseError::UnexpectedToken(
                span,
                GtNode::Primitive,
                token.kind,
                "expected primitive kind",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_parse() {
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "boolean")),
            @"Boolean"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "string")),
            @"String"
        );

        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "number")),
            @"Number"
        );
    }

    #[test]
    fn test_int_sizes() {
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "i8")),
            @"Int8"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "i16")),
            @"Int16"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "i32")),
            @"Int32"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "i64")),
            @"Int64"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "i128")),
            @"Int128"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "isize")),
            @"IntSize"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "u8")),
            @"IntU8"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "u16")),
            @"IntU16"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "u32")),
            @"IntU32"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "u64")),
            @"IntU64"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "u128")),
            @"IntU128"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(GtTokenKind::Primitive, "usize")),
            @"IntUSize"
        );
    }

    #[test]
    fn test_error() {
        assert_debug_snapshot!(
            parse_node_err!(GtPrimitiveKind, to_parse_args(GtTokenKind::LiteralBoolean, "false")),
            @r#"
        UnexpectedToken(
            GtSpan(
                0,
                5,
            ),
            Primitive,
            literal_boolean,
            "expected primitive kind",
        )
        "#
        );
    }
}
