use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TSArray {
    pub descriptor: TSDescriptor,
}
