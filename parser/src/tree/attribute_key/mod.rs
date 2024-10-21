use crate::GTSpan;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTAttributeKey {
    pub span: GTSpan,
    pub name: String,
}

impl GTAttributeKey {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self { span, name }
    }
}
