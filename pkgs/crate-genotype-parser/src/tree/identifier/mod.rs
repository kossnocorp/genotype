use crate::prelude::internal::*;

mod parse;

/// Unique module identifier.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTIdentifier(
    /// Identifier position in the source code.
    pub GTSpan,
    /// Identifier name.
    pub Arc<str>,
);

impl GTIdentifier {
    pub fn new(span: GTSpan, name: Arc<str>) -> Self {
        Self(span, name)
    }

    pub fn as_span(&self) -> GTSpan {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        self.1.as_ref()
    }

    pub fn as_string(&self) -> String {
        self.1.to_string()
    }
}
