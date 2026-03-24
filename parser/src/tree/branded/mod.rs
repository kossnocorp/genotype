use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTBranded {
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    pub id: GTDefinitionId,
    #[visit]
    pub name: GTIdentifier,
    #[visit]
    pub primitive: GTPrimitive,
}
