use crate::GTSpan;

use super::{descriptor::GTDescriptor, doc::GTDoc, key::GTKey};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProperty {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub name: GTKey,
    pub descriptor: GTDescriptor,
    pub required: bool,
}
