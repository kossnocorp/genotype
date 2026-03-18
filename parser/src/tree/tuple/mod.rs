use serde::Serialize;

use crate::GTSpan;

use super::descriptor::GTDescriptor;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTTuple {
    pub span: GTSpan,
    pub descriptors: Vec<GTDescriptor>,
}
