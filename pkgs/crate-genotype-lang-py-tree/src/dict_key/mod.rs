use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum PyDictKey {
    Int,
    Float,
    String,
    Boolean,
}
