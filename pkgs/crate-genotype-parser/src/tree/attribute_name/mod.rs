use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTAttributeName {
    pub span: GTSpan,
    pub value: Arc<str>,
}

impl GTAttributeName {
    pub fn new(span: GTSpan, value: Arc<str>) -> Self {
        Self { span, value }
    }
}
