use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTAttributeKey {
    pub span: GTSpan,
    pub value: Arc<str>,
}

impl GTAttributeKey {
    pub fn new(span: GTSpan, name: Arc<str>) -> Self {
        Self { span, value: name }
    }
}
