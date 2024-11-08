use crate::descriptor::RSDescriptor;

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSUnion {
    pub descriptors: Vec<RSDescriptor>,
    pub discriminator: Option<String>,
}
