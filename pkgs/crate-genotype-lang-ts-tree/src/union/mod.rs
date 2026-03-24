use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSUnion {
    #[visit]
    pub descriptors: Vec<TSDescriptor>,
}
