use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSOption {
    #[visit]
    pub descriptor: RSDescriptor,
}

impl RSOption {
    pub fn new(descriptor: RSDescriptor) -> Self {
        Self { descriptor }
    }
}
