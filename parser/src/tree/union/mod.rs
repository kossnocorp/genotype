use super::descriptor::GTDescriptor;

#[derive(Debug, PartialEq, Clone)]
pub struct GTUnion {
    pub descriptors: Vec<GTDescriptor>,
}
