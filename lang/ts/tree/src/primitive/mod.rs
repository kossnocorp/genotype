use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TSPrimitive {
    String,
    Number,
    BigInt,
    Boolean,
    Null,
    Undefined,
}
