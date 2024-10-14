use crate::GTSpan;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTIdentifier(pub GTSpan, pub String);

impl GTIdentifier {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self(span, name)
    }
}
