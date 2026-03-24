use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYUnion {
    #[visit]
    pub descriptors: Vec<PYDescriptor>,
    pub discriminator: Option<String>,
}
