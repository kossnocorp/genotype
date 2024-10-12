use crate::descriptor::TSDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSIntersection {
    pub descriptors: Vec<TSDescriptor>,
}
