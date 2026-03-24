use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYNewtype {
    #[visit]
    pub doc: Option<PYDoc>,
    #[visit]
    pub name: PYIdentifier,
    #[visit]
    pub primitive: PYPrimitive,
}
