use crate::descriptor::RSDescriptor;

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSTuple {
    pub descriptors: Vec<RSDescriptor>,
}
