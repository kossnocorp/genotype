use crate::descriptor::PYDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYTuple {
    pub descriptors: Vec<PYDescriptor>,
}
