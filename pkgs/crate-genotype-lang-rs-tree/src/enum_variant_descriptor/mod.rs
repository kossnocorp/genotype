use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum RsEnumVariantDescriptor {
    Descriptor(#[visit] RsDescriptor),
    // [TODO] Add inline struct and Vec<RsDescriptor>
}

impl From<RsDescriptor> for RsEnumVariantDescriptor {
    fn from(descriptor: RsDescriptor) -> Self {
        RsEnumVariantDescriptor::Descriptor(descriptor)
    }
}
