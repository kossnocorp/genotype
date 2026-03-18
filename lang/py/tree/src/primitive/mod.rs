use serde::Serialize;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum PYPrimitive {
    Boolean,
    String,
    Int,
    Float,
    None,
}
