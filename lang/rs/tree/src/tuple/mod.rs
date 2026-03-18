use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RSTuple {
    pub descriptors: Vec<RSDescriptor>,
}
