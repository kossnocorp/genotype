use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TSIntersection {
    pub descriptors: Vec<TSDescriptor>,
}
