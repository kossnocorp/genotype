use std::hash::Hash;

use crate::GTSpan;

use super::GTPathModuleId;

mod parse;

/// Unique module path identifier.
#[derive(Debug, Eq, Hash, Clone)]
pub struct GTPath(
    /// Where the path is defined in the source code.
    pub GTSpan,
    /// Module identifier. May be unresolved.
    pub GTPathModuleId,
    /// Literal path string.
    String,
);

impl GTPath {
    pub fn new(span: GTSpan, module_id: GTPathModuleId, path: String) -> Self {
        Self(span, module_id, path)
    }

    pub fn as_str(&self) -> &str {
        &self.2
    }
}

impl PartialEq for GTPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.2 == other.2
    }
}
