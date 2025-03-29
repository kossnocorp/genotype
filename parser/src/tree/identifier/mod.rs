use crate::GTSpan;

mod parse;

/// Unique module identifier.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTIdentifier(
    /// Identifier position in the source code.
    pub GTSpan,
    /// Identifier name.
    pub String,
);

impl GTIdentifier {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self(span, name)
    }

    pub fn as_span(&self) -> GTSpan {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.1
    }

    pub fn as_string(&self) -> String {
        self.1.clone()
    }
}
