use super::{descriptor::GTDescriptor, name::GTName};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProperty {
    pub doc: Option<String>,
    pub name: GTName,
    pub descriptor: GTDescriptor,
    pub required: bool,
}
