use crate::type_descriptor::TSTypeDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSTuple {
    pub descriptors: Vec<TSTypeDescriptor>,
}
