use crate::GTSpan;

use super::*;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAlias {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub name: GTIdentifier,
    pub descriptor: GTDescriptor,
}
