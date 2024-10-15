use crate::GTSpan;

use super::descriptor::GTDescriptor;

#[derive(Debug, PartialEq, Clone)]
pub struct GTUnion {
    pub span: GTSpan,
    pub descriptors: Vec<GTDescriptor>,
}
