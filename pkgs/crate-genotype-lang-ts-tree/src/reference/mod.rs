use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsReference(pub TsIdentifier);

impl From<&str> for TsReference {
    fn from(str: &str) -> Self {
        TsReference(str.into())
    }
}

impl From<TsIdentifier> for TsReference {
    fn from(identifier: TsIdentifier) -> Self {
        TsReference(identifier)
    }
}
