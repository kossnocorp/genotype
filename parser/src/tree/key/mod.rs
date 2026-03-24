use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTKey(pub GTSpan, pub String);

impl GTKey {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self(span, name)
    }
}
