use crate::{descriptor::PYDescriptor, PYDictKey};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYDict {
    pub key: PYDictKey,
    pub descriptor: PYDescriptor,
}
