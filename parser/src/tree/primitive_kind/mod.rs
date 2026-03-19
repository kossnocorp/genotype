use crate::prelude::internal::*;

mod parse;

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
