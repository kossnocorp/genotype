use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYList {
    #[visit]
    pub descriptor: PYDescriptor,
}
