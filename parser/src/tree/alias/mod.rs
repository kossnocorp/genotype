use crate::GTSpan;

use super::{descriptor::GTDescriptor, doc::GTDoc, identifier::GTIdentifier};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAlias {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub name: GTIdentifier,
    pub descriptor: GTDescriptor,
}
