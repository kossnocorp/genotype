use crate::type_descriptor::TSTypeDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSUnion {
    pub descriptors: Vec<TSTypeDescriptor>,
}
