use super::{descriptor::GTDescriptor, doc::GTDoc, identifier::GTIdentifier};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAlias {
    pub doc: Option<GTDoc>,
    pub name: GTIdentifier,
    pub descriptor: GTDescriptor,
}
