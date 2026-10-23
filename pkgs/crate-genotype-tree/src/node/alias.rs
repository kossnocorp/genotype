use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAlias {
    pub id: GtDefinitionId,
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub name: GtIdentifier,
    #[visit]
    pub generics: Vec<GtGenericParameter>,
    #[visit]
    pub descriptor: GtDescriptor,
}
