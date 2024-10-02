use crate::{descriptor::TSDescriptor, key::TSKey};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSProperty {
    pub name: TSKey,
    pub descriptor: TSDescriptor,
    pub required: bool,
}
