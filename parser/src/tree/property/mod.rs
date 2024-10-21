use crate::GTSpan;

use super::{descriptor::GTDescriptor, doc::GTDoc, key::GTKey, GTAttribute};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTProperty {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub attributes: Vec<GTAttribute>,
    pub name: GTKey,
    pub descriptor: GTDescriptor,
    pub required: bool,
}
