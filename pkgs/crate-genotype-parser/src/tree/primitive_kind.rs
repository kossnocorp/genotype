use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub enum GtPrimitiveKind {
    Boolean,
    String,
    Number,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    IntSize,
    IntU8,
    IntU16,
    IntU32,
    IntU64,
    IntU128,
    IntUSize,
    Float32,
    Float64,
}

impl Display for GtPrimitiveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GtPrimitiveKind::Boolean => write!(f, "bool"),
            GtPrimitiveKind::String => write!(f, "str"),
            GtPrimitiveKind::Number => write!(f, "number"),
            GtPrimitiveKind::Int8 => write!(f, "i8"),
            GtPrimitiveKind::Int16 => write!(f, "i16"),
            GtPrimitiveKind::Int32 => write!(f, "i32"),
            GtPrimitiveKind::Int64 => write!(f, "i64"),
            GtPrimitiveKind::Int128 => write!(f, "i128"),
            GtPrimitiveKind::IntSize => write!(f, "isize"),
            GtPrimitiveKind::IntU8 => write!(f, "u8"),
            GtPrimitiveKind::IntU16 => write!(f, "u16"),
            GtPrimitiveKind::IntU32 => write!(f, "u32"),
            GtPrimitiveKind::IntU64 => write!(f, "u64"),
            GtPrimitiveKind::IntU128 => write!(f, "u128"),
            GtPrimitiveKind::IntUSize => write!(f, "usize"),
            GtPrimitiveKind::Float32 => write!(f, "f32"),
            GtPrimitiveKind::Float64 => write!(f, "f64"),
        }
    }
}

impl GtPrimitiveKind {
    pub fn parse(pair: Pair<'_, Rule>, _context: &mut GtContext) -> GtNodeParseResult<Self> {
        let span = pair.as_span().into();
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
