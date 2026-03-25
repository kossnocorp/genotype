use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsPrimitive {
    String,
    Number,
    BigInt,
    Boolean,
    Null,
    Undefined,
}
