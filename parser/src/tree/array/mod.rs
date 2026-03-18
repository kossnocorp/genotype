use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTArray {
    pub span: GTSpan,
    pub descriptor: GTDescriptor,
}
