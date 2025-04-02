use std::hash::Hash;

use crate::GTSpan;

use super::GTPathModuleId;

mod parse;

/// Unique module path reference. It defines a particular path reference in the source code.
#[derive(Debug, Eq, Hash, Clone)]
pub struct GTPath(
    /// Where the path is defined in the source code.
    pub GTSpan,
    /// Module identifier. May be unresolved.
    pub GTPathModuleId,
    /// Literal path string how it was defined in the source code.
    String,
);

impl GTPath {
    pub fn new(span: GTSpan, module_id: GTPathModuleId, path: String) -> Self {
        Self(span, module_id, path)
    }

    /// Returns the literal path string ref.
    pub fn source_str(&self) -> &str {
        &self.2
    }

    pub fn kind(&self) -> GTPathKind {
        if self.2.starts_with('.') || self.2.starts_with("~") {
            GTPathKind::Local
        } else {
            GTPathKind::Package
        }
    }

    pub fn package_path(&self) -> Option<(String, Option<String>)> {
        if self.kind() == GTPathKind::Package {
            Some(match self.2.find("/") {
                Some(index) => (
                    self.2[..index].to_owned(),
                    Some(self.2[index + 1..].to_owned()),
                ),
                None => (self.2.clone(), None),
            })
        } else {
            None
        }
    }
}

impl PartialEq for GTPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.2 == other.2
    }
}

#[derive(PartialEq)]
pub enum GTPathKind {
    /// Local path (starts with `.`, `..` or `~/`).
    Local,
    /// Package path (have to prefix).
    Package,
}
