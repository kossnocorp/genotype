use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSOption {
    pub descriptor: RSDescriptor,
}

impl RSOption {
    pub fn new(descriptor: RSDescriptor) -> Self {
        Self { descriptor }
    }
}
