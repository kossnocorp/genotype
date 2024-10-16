use crate::{descriptor::PYDescriptor, identifier::PYIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYAlias {
    pub name: PYIdentifier,
    pub descriptor: PYDescriptor,
}
