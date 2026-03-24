use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSIdentifier(pub Arc<str>);

impl GtlDependencyRef for TSIdentifier {}

impl From<&str> for TSIdentifier {
    fn from(str: &str) -> Self {
        TSIdentifier(str.into())
    }
}

impl From<String> for TSIdentifier {
    fn from(str: String) -> Self {
        TSIdentifier(str.into())
    }
}

impl From<Arc<str>> for TSIdentifier {
    fn from(str: Arc<str>) -> Self {
        TSIdentifier(str)
    }
}
