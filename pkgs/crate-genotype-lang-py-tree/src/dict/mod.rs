use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYDict {
    #[visit]
    pub key: PYDictKey,
    #[visit]
    pub descriptor: PYDescriptor,
}
