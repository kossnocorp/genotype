use serde::Serialize;

use crate::GTSpan;

use super::descriptor::GTDescriptor;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTArray {
    pub span: GTSpan,
    pub descriptor: GTDescriptor,
}
