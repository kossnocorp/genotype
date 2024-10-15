use crate::GTSpan;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTDoc(pub GTSpan, pub String);

impl GTDoc {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self(span, name)
    }
}
