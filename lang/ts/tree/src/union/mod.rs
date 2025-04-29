use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSUnion {
    pub descriptors: Vec<TSDescriptor>,
}
