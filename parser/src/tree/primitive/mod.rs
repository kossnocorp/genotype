use std::fmt::Display;

use crate::diagnostic::span::GTSpan;

use super::GTDescriptor;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub enum GTPrimitive {
    Boolean(GTSpan),
    String(GTSpan),
    Number(GTSpan),
    Int8(GTSpan),
    Int16(GTSpan),
    Int32(GTSpan),
    Int64(GTSpan),
    Int128(GTSpan),
    IntSize(GTSpan),
    IntU8(GTSpan),
    IntU16(GTSpan),
    IntU32(GTSpan),
    IntU64(GTSpan),
    IntU128(GTSpan),
    IntUSize(GTSpan),
    Float32(GTSpan),
    Float64(GTSpan),
    Null(GTSpan),
}

impl Display for GTPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GTPrimitive::Boolean(_) => write!(f, "bool"),
            GTPrimitive::String(_) => write!(f, "str"),
            GTPrimitive::Number(_) => write!(f, "number"),
            GTPrimitive::Int8(_) => write!(f, "i8"),
            GTPrimitive::Int16(_) => write!(f, "i16"),
            GTPrimitive::Int32(_) => write!(f, "i32"),
            GTPrimitive::Int64(_) => write!(f, "i64"),
            GTPrimitive::Int128(_) => write!(f, "i128"),
            GTPrimitive::IntSize(_) => write!(f, "isize"),
            GTPrimitive::IntU8(_) => write!(f, "u8"),
            GTPrimitive::IntU16(_) => write!(f, "u16"),
            GTPrimitive::IntU32(_) => write!(f, "u32"),
            GTPrimitive::IntU64(_) => write!(f, "u64"),
            GTPrimitive::IntU128(_) => write!(f, "u128"),
            GTPrimitive::IntUSize(_) => write!(f, "usize"),
            GTPrimitive::Float32(_) => write!(f, "f32"),
            GTPrimitive::Float64(_) => write!(f, "f64"),
            GTPrimitive::Null(_) => write!(f, "null"),
        }
    }
}

impl Into<GTDescriptor> for GTPrimitive {
    fn into(self) -> GTDescriptor {
        GTDescriptor::Primitive(self)
    }
}
