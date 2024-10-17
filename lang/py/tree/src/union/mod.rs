use crate::descriptor::PYDescriptor;

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYUnion {
    pub descriptors: Vec<PYDescriptor>,
}
