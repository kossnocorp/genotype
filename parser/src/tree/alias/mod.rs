use super::{descriptor::GTDescriptor, name::GTName};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAlias {
    pub doc: Option<String>,
    pub name: GTName,
    pub descriptor: GTDescriptor,
}
