use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYImportReference {
    Default(Option<PYIdentifier>),
    Glob,
    Named(Vec<PYImportName>),
}

impl From<&str> for PYImportReference {
    fn from(str: &str) -> Self {
        PYImportReference::Named(vec![str.into()])
    }
}
