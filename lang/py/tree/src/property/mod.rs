use crate::{descriptor::PYDescriptor, key::PYKey, PYDoc};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYProperty {
    pub doc: Option<PYDoc>,
    pub name: PYKey,
    pub descriptor: PYDescriptor,
    pub required: bool,
}
