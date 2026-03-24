use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSTuple {
    #[visit]
    pub descriptors: Vec<RSDescriptor>,
}
