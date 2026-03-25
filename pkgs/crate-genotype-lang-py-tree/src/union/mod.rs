use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyUnion {
    #[visit]
    pub descriptors: Vec<PyDescriptor>,
    pub discriminator: Option<String>,
}
