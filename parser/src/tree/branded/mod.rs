use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTBranded {
    pub span: GTSpan,
    pub id: GTDefinitionId,
    pub name: GTIdentifier,
    pub primitive: GTPrimitive,
}
