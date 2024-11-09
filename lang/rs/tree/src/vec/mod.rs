use crate::descriptor::RSDescriptor;

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSVec {
    pub descriptor: RSDescriptor,
}
