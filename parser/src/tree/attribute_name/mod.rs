use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GTAttributeName {
    pub span: GTSpan,
    pub name: String,
}

impl GTAttributeName {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self { span, name }
    }
}
