use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GTPrimitiveKind {
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

impl Display for GTPrimitiveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GTPrimitiveKind::Boolean => write!(f, "bool"),
            GTPrimitiveKind::String => write!(f, "str"),
            GTPrimitiveKind::Number => write!(f, "number"),
            GTPrimitiveKind::Int8 => write!(f, "i8"),
            GTPrimitiveKind::Int16 => write!(f, "i16"),
            GTPrimitiveKind::Int32 => write!(f, "i32"),
            GTPrimitiveKind::Int64 => write!(f, "i64"),
            GTPrimitiveKind::Int128 => write!(f, "i128"),
            GTPrimitiveKind::IntSize => write!(f, "isize"),
            GTPrimitiveKind::IntU8 => write!(f, "u8"),
            GTPrimitiveKind::IntU16 => write!(f, "u16"),
            GTPrimitiveKind::IntU32 => write!(f, "u32"),
            GTPrimitiveKind::IntU64 => write!(f, "u64"),
            GTPrimitiveKind::IntU128 => write!(f, "u128"),
            GTPrimitiveKind::IntUSize => write!(f, "usize"),
            GTPrimitiveKind::Float32 => write!(f, "f32"),
            GTPrimitiveKind::Float64 => write!(f, "f64"),
        }
    }
}

impl GTPrimitiveKind {
    pub fn parse(pair: Pair<'_, Rule>, context: &mut GTContext) -> GTNodeParseResult<Self> {
        let span = pair.as_span().into();
        match pair.as_str() {
            "boolean" => Ok(GTPrimitiveKind::Boolean),
            "string" => Ok(GTPrimitiveKind::String),
            "number" => Ok(GTPrimitiveKind::Number),
            "int" => Ok(GTPrimitiveKind::Int64),
            "i8" => Ok(GTPrimitiveKind::Int8),
            "i16" => Ok(GTPrimitiveKind::Int16),
            "i32" => Ok(GTPrimitiveKind::Int32),
            "i64" => Ok(GTPrimitiveKind::Int64),
            "i128" => Ok(GTPrimitiveKind::Int128),
            "isize" => Ok(GTPrimitiveKind::IntSize),
            "uint" => Ok(GTPrimitiveKind::IntU32),
            "u8" => Ok(GTPrimitiveKind::IntU8),
            "u16" => Ok(GTPrimitiveKind::IntU16),
            "u32" => Ok(GTPrimitiveKind::IntU32),
            "u64" => Ok(GTPrimitiveKind::IntU64),
            "u128" => Ok(GTPrimitiveKind::IntU128),
            "usize" => Ok(GTPrimitiveKind::IntUSize),
            "float" => Ok(GTPrimitiveKind::Float64),
            "f32" => Ok(GTPrimitiveKind::Float32),
            "f64" => Ok(GTPrimitiveKind::Float64),
            _ => Err(GTParseError::Internal(span, GTNode::Primitive)),
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
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "boolean")),
            @"Boolean"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "string")),
            @"String"
        );

        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "number")),
            @"Number"
        );
    }

    #[test]
    fn test_int_sizes() {
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "i8")),
            @"Int8"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "i16")),
            @"Int16"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "i32")),
            @"Int32"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "i64")),
            @"Int64"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "i128")),
            @"Int128"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "isize")),
            @"IntSize"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "u8")),
            @"IntU8"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "u16")),
            @"IntU16"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "u32")),
            @"IntU32"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "u64")),
            @"IntU64"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "u128")),
            @"IntU128"
        );
        assert_ron_snapshot!(
            parse_node!(GTPrimitiveKind, to_parse_args(Rule::primitive, "usize")),
            @"IntUSize"
        );
    }

    #[test]
    fn test_error() {
        assert_debug_snapshot!(
            parse_node_err!(GTPrimitiveKind, to_parse_args(Rule::literal_boolean, "false")),
            @"
        Internal(
            GTSpan(
                0,
                5,
            ),
            Primitive,
        )
        "
        );
    }
}
