use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord, Serialize, Visitor)]
pub struct PyIdentifier(pub Arc<str>);

impl GtlDependencyRef for PyIdentifier {}

impl From<&str> for PyIdentifier {
    fn from(str: &str) -> Self {
        PyIdentifier(str.into())
    }
}

impl From<String> for PyIdentifier {
    fn from(str: String) -> Self {
        PyIdentifier(str.into())
    }
}

impl From<Arc<str>> for PyIdentifier {
    fn from(str: Arc<str>) -> Self {
        PyIdentifier(str)
    }
}
