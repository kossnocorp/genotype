use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
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
