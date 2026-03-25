use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsAlias {
    #[visit]
    pub doc: Option<TsDoc>,
    #[visit]
    pub name: TsIdentifier,
    #[visit]
    pub descriptor: TsDescriptor,
}
