use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord, Serialize, Visitor)]
pub struct RsIdentifier(pub Arc<str>);

impl GtlDependencyRef for RsIdentifier {}

impl From<&str> for RsIdentifier {
    fn from(str: &str) -> Self {
        RsIdentifier(str.into())
    }
}

impl From<String> for RsIdentifier {
    fn from(str: String) -> Self {
        RsIdentifier(str.into())
    }
}

impl From<Arc<str>> for RsIdentifier {
    fn from(str: Arc<str>) -> Self {
        RsIdentifier(str)
    }
}
