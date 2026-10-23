use crate::prelude::internal::*;

/// Unique module path reference. It defines a particular path reference in the source code.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtPath {
    /// Where the path is defined in the source code.
    pub span: GtSpan,
    /// Path id.
    pub id: GtPathModuleId,
    /// Literal path string how it was defined in the source code.
    pub path: Arc<str>,
}

impl GtPath {
    pub fn new(span: GtSpan, module_id: GtPathModuleId, path: Arc<str>) -> Self {
        Self {
            span,
            id: module_id,
            path,
        }
    }

    /// Returns the literal path string ref.
    pub fn source_str(&self) -> &str {
        self.path.as_ref()
    }

    pub fn kind(&self) -> GtPathKind {
        if self.path.starts_with('.') || self.path.starts_with("~") {
            GtPathKind::Local
        } else {
            GtPathKind::Package
        }
    }

    pub fn package_path(&self) -> Option<(String, Option<String>)> {
        if self.kind() == GtPathKind::Package {
            Some(match self.path.find("/") {
                Some(index) => (
                    self.path[..index].to_owned(),
                    Some(self.path[index + 1..].to_owned()),
                ),
                None => (self.path.to_string(), None),
            })
        } else {
            None
        }
    }
}

#[derive(PartialEq)]
pub enum GtPathKind {
    /// Local path (starts with `.`, `..` or `~/`).
    Local,
    /// Package path (have to prefix).
    Package,
}
