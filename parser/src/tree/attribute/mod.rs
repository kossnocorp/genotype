use crate::GTSpan;

use super::{GTAttributeDescriptor, GTAttributeName};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAttribute {
    pub span: GTSpan,
    pub name: GTAttributeName,
    pub descriptor: Option<GTAttributeDescriptor>,
}
