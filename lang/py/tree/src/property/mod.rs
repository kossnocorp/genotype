use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PYProperty {
    pub doc: Option<PYDoc>,
    pub name: PYKey,
    pub descriptor: PYDescriptor,
    pub required: bool,
}
