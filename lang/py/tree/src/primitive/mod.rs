mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYPrimitive {
    Boolean,
    String,
    Int,
    Float,
    None,
}
