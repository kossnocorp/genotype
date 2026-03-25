use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsBranded {
    #[visit]
    pub doc: Option<TsDoc>,
    #[visit]
    pub name: TsIdentifier,
    #[visit]
    pub primitive: TsPrimitive,
}
