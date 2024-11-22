use crate::diagnostic::span::GTSpan;

use super::{GTDefinitionId, GTDoc, GTIdentifier, GTPrimitive};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTBranded {
    pub span: GTSpan,
    pub id: GTDefinitionId,
    pub name: GTIdentifier,
    pub primitive: GTPrimitive,
}
