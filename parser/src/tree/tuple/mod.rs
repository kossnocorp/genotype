use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTTuple {
    pub span: GTSpan,
    pub descriptors: Vec<GTDescriptor>,
}
