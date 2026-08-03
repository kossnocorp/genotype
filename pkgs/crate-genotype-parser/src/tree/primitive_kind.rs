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
