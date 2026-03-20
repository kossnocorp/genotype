use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GTAttributeName {
    pub span: GTSpan,
    pub value: String,
}

impl GTAttributeName {
    pub fn new(span: GTSpan, value: String) -> Self {
        Self { span, value }
    }
}
