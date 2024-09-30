use crate::type_descriptor::TSTypeDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSArray {
    pub descriptor: TSTypeDescriptor,
}
