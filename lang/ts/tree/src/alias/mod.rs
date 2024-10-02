use crate::{descriptor::TSDescriptor, identifier::TSIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSAlias {
    pub name: TSIdentifier,
    pub descriptor: TSDescriptor,
}
