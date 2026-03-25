use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsIntersection {
    #[visit]
    pub descriptors: Vec<TsDescriptor>,
}
