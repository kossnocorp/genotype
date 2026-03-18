use serde::Serialize;

use crate::GTSpan;

use super::descriptor::GTDescriptor;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTUnion {
    pub span: GTSpan,
    pub descriptors: Vec<GTDescriptor>,
}
