use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum PyImportName {
    Name(#[visit] PyIdentifier),
    Alias(#[visit] PyIdentifier, #[visit] PyIdentifier),
}

impl GtlImportRefName for PyImportName {}

impl From<&str> for PyImportName {
    fn from(str: &str) -> Self {
        PyImportName::Name(str.into())
    }
}

impl From<PyIdentifier> for PyImportName {
    fn from(identifier: PyIdentifier) -> Self {
        PyImportName::Name(identifier)
    }
}
