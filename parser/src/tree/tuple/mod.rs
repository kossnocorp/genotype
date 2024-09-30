use super::descriptor::GTDescriptor;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTTuple {
    pub descriptors: Vec<GTDescriptor>,
}
