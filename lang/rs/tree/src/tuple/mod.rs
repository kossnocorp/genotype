use crate::descriptor::RSDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSTuple {
    pub descriptors: Vec<RSDescriptor>,
}
