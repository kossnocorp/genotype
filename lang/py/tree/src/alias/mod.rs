use crate::{descriptor::PYDescriptor, identifier::PYIdentifier, PYDoc};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYAlias {
    pub doc: Option<PYDoc>,
    pub name: PYIdentifier,
    pub descriptor: PYDescriptor,
}
