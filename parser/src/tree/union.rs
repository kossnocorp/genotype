use super::descriptor::GTDescriptor;

#[derive(Debug, PartialEq)]
pub struct GTUnion {
    pub descriptors: Vec<GTDescriptor>,
}
