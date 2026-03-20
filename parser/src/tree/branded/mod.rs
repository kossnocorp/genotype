use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTBranded {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub attributes: Vec<GTAttribute>,
    pub id: GTDefinitionId,
    pub name: GTIdentifier,
    pub primitive: GTPrimitive,
}
