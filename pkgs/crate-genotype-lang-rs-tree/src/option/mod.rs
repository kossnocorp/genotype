use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsOption {
    #[visit]
    pub descriptor: RsDescriptor,
}

impl RsOption {
    pub fn new(descriptor: RsDescriptor) -> Self {
        Self { descriptor }
    }
}
