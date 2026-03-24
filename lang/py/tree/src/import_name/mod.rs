use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum PYImportName {
    Name(#[visit] PYIdentifier),
    Alias(#[visit] PYIdentifier, #[visit] PYIdentifier),
}

impl From<&str> for PYImportName {
    fn from(str: &str) -> Self {
        PYImportName::Name(str.into())
    }
}

impl From<PYIdentifier> for PYImportName {
    fn from(identifier: PYIdentifier) -> Self {
        PYImportName::Name(identifier)
    }
}
