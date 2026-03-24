use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum PYImportReference {
    Default(#[visit] Option<PYIdentifier>),
    Glob,
    Named(#[visit] Vec<PYImportName>),
}

impl From<&str> for PYImportReference {
    fn from(str: &str) -> Self {
        PYImportReference::Named(vec![str.into()])
    }
}
