use crate::GTSpan;

use super::{GTAttributeKey, GTAttributeValue};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAttributeProperty {
    pub span: GTSpan,
    pub name: GTAttributeKey,
    pub value: GTAttributeValue,
}

impl GTAttributeProperty {
    pub fn new(span: GTSpan, name: GTAttributeKey, value: GTAttributeValue) -> Self {
        Self { span, name, value }
    }
}
