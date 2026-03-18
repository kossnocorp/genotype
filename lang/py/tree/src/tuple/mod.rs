use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PYTuple {
    pub descriptors: Vec<PYDescriptor>,
}
