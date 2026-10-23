use crate::prelude::internal::*;

impl GtParse for GtPrimitiveKind {
    fn parse(pair: Pair<'_, Rule>, _context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = (pair.as_span().start(), pair.as_span().end()).into();
        match pair.as_str() {
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
            _ => Err(GtParseError::UnexpectedRule(
                span,
                GtNode::Primitive,
                pair.as_rule(),
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
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "boolean")),
            @"Boolean"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "string")),
            @"String"
        );

        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "number")),
            @"Number"
        );
    }

    #[test]
    fn test_int_sizes() {
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "i8")),
            @"Int8"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "i16")),
            @"Int16"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "i32")),
            @"Int32"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "i64")),
            @"Int64"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "i128")),
            @"Int128"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "isize")),
            @"IntSize"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "u8")),
            @"IntU8"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "u16")),
            @"IntU16"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "u32")),
            @"IntU32"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "u64")),
            @"IntU64"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "u128")),
            @"IntU128"
        );
        assert_ron_snapshot!(
            parse_node!(GtPrimitiveKind, to_parse_args(Rule::primitive, "usize")),
            @"IntUSize"
        );
    }

    #[test]
    fn test_error() {
        assert_debug_snapshot!(
            parse_node_err!(GtPrimitiveKind, to_parse_args(Rule::literal_boolean, "false")),
            @r#"
        UnexpectedRule(
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
