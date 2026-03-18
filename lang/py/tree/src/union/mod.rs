use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PYUnion {
    pub descriptors: Vec<PYDescriptor>,
    pub discriminator: Option<String>,
}
