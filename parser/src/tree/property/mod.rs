use super::{descriptor::GTDescriptor, doc::GTDoc, key::GTKey};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProperty {
    pub doc: Option<GTDoc>,
    pub name: GTKey,
    pub descriptor: GTDescriptor,
    pub required: bool,
}
