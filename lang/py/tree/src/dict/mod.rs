use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYDict {
    pub key: PYDictKey,
    pub descriptor: PYDescriptor,
}
