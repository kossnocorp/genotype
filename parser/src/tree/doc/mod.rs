use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTDoc(pub GTSpan, pub String);

impl GTDoc {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self(span, name)
    }
}
