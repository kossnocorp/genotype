use crate::descriptor::TSDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSUnion {
    pub descriptors: Vec<TSDescriptor>,
}
