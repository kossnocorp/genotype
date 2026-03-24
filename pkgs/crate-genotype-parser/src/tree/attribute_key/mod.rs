use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTAttributeKey {
    pub span: GTSpan,
    pub value: String,
}

impl GTAttributeKey {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self { span, value: name }
    }
}
