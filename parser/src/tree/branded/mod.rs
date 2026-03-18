use serde::Serialize;

use crate::diagnostic::span::GTSpan;

use super::{GTDefinitionId, GTIdentifier, GTPrimitive};

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTBranded {
    pub span: GTSpan,
    pub id: GTDefinitionId,
    pub name: GTIdentifier,
    pub primitive: GTPrimitive,
}
