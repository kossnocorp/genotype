use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsIdentifier(pub Arc<str>);

impl GtlDependencyRef for TsIdentifier {}

impl From<&str> for TsIdentifier {
    fn from(str: &str) -> Self {
        TsIdentifier(str.into())
    }
}

impl From<String> for TsIdentifier {
    fn from(str: String) -> Self {
        TsIdentifier(str.into())
    }
}

impl From<Arc<str>> for TsIdentifier {
    fn from(str: Arc<str>) -> Self {
        TsIdentifier(str)
    }
}
