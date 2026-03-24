use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSIntersection {
    #[visit]
    pub descriptors: Vec<TSDescriptor>,
}
