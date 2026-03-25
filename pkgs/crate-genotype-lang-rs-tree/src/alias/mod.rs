use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsAlias {
    pub id: GtDefinitionId,
    #[visit]
    pub doc: Option<RsDoc>,
    #[visit]
    pub name: RsIdentifier,
    #[visit]
    pub descriptor: RsDescriptor,
}
