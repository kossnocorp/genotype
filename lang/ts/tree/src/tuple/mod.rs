use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSTuple {
    pub descriptors: Vec<TSDescriptor>,
}
