use crate::descriptor::PYDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYList {
    pub descriptor: PYDescriptor,
}
