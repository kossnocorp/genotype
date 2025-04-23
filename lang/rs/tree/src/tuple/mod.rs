use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSTuple {
    pub descriptors: Vec<RSDescriptor>,
}
