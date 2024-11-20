use crate::descriptor::RSDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSMap {
    pub key: RSDescriptor,
    pub descriptor: RSDescriptor,
}
