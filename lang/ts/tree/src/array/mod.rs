use crate::descriptor::TSDescriptor;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSArray {
    pub descriptor: TSDescriptor,
}
