use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYAlias {
    #[visit]
    pub doc: Option<PYDoc>,
    #[visit]
    pub name: PYIdentifier,
    #[visit]
    pub descriptor: PYDescriptor,
    #[visit]
    pub references: Vec<PYIdentifier>,
}
