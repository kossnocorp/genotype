use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYProperty {
    #[visit]
    pub doc: Option<PYDoc>,
    #[visit]
    pub name: PYKey,
    #[visit]
    pub descriptor: PYDescriptor,
    pub required: bool,
}
