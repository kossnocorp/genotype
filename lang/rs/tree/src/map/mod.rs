use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSMap {
    pub key: RSDescriptor,
    pub descriptor: RSDescriptor,
}
