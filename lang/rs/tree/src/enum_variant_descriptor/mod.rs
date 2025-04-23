use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSEnumVariantDescriptor {
    Descriptor(RSDescriptor),
    // [TODO] Add inline stuct and Vec<RSDescriptor>
}

impl From<RSDescriptor> for RSEnumVariantDescriptor {
    fn from(descriptor: RSDescriptor) -> Self {
        RSEnumVariantDescriptor::Descriptor(descriptor)
    }
}
