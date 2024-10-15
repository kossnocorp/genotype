use crate::GTSpan;

use super::descriptor::GTDescriptor;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTArray {
    pub span: GTSpan,
    pub descriptor: GTDescriptor,
}
