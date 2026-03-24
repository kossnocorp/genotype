use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum RSEnumVariantDescriptor {
    Descriptor(#[visit] RSDescriptor),
    // [TODO] Add inline struct and Vec<RSDescriptor>
}

impl From<RSDescriptor> for RSEnumVariantDescriptor {
    fn from(descriptor: RSDescriptor) -> Self {
        RSEnumVariantDescriptor::Descriptor(descriptor)
    }
}
