use crate::descriptor::RSDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSHashMap {
    pub key: RSDescriptor,
    pub descriptor: RSDescriptor,
}
