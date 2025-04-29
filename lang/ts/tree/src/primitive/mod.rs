mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSPrimitive {
    String,
    Number,
    BigInt,
    Boolean,
    Null,
    Undefined,
}
