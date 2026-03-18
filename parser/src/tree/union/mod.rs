use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTUnion {
    pub span: GTSpan,
    pub descriptors: Vec<GTDescriptor>,
}
