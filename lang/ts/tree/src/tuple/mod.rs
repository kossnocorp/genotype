use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSTuple {
    pub descriptors: Vec<TSDescriptor>,
}
