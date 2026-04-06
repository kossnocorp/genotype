use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum PyImportReference {
    Default(#[visit] Option<PyIdentifier>),
    Glob,
    Named(#[visit] Vec<PyImportName>),
}

impl GtlImportRef for PyImportReference {}

impl From<&str> for PyImportReference {
    fn from(str: &str) -> Self {
        PyImportReference::Named(vec![str.into()])
    }
}

impl From<PyIdentifier> for PyImportReference {
    fn from(identifier: PyIdentifier) -> Self {
        PyImportReference::Named(vec![identifier.into()])
    }
}
