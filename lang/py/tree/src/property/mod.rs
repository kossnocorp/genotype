use crate::{descriptor::PYDescriptor, key::PYKey};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYProperty {
    pub name: PYKey,
    pub descriptor: PYDescriptor,
    pub required: bool,
}
