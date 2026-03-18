use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTAttributeKey {
    pub span: GTSpan,
    pub name: String,
}

impl GTAttributeKey {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self { span, name }
    }
}
