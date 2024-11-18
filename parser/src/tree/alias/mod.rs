use crate::GTSpan;

use super::*;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAlias {
    pub id: GTDefinitionId,
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub attributes: Vec<GTAttribute>,
    pub name: GTIdentifier,
    pub descriptor: GTDescriptor,
}
