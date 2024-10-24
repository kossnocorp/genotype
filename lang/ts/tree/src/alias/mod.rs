use crate::{descriptor::TSDescriptor, identifier::TSIdentifier, TSDoc};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSAlias {
    pub doc: Option<TSDoc>,
    pub name: TSIdentifier,
    pub descriptor: TSDescriptor,
}
