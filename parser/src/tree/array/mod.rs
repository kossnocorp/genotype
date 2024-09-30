use super::descriptor::GTDescriptor;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTArray {
    pub descriptor: GTDescriptor,
}
