use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsStruct {
    pub id: GtDefinitionId,
    #[visit]
    pub doc: Option<RsDoc>,
    #[visit]
    pub attributes: Vec<RsAttribute>,
    #[visit]
    pub name: RsIdentifier,
    #[visit]
    pub fields: RsStructFields,
}
