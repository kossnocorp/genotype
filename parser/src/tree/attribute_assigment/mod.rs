use crate::GTSpan;

use super::GTAttributeValue;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAttributeAssignment {
    pub span: GTSpan,
    pub value: GTAttributeValue,
}

impl GTAttributeAssignment {
    pub fn new(span: GTSpan, value: GTAttributeValue) -> Self {
        Self { span, value }
    }
}
