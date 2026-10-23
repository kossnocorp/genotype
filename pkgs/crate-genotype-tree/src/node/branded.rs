use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtBranded {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    pub id: GtDefinitionId,
    #[visit]
    pub name: GtIdentifier,
    #[visit]
    pub primitive: GtPrimitive,
}
