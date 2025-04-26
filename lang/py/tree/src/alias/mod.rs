use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYAlias {
    pub doc: Option<PYDoc>,
    pub name: PYIdentifier,
    pub descriptor: PYDescriptor,
    pub references: Vec<PYIdentifier>,
}
