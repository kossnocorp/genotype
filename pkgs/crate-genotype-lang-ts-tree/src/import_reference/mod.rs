use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsImportReference {
    Default(Arc<str>),
    Glob(Arc<str>),
    Named(#[visit] Vec<TsImportName>),
}

impl From<&str> for TsImportReference {
    fn from(str: &str) -> Self {
        TsImportReference::Named(vec![str.into()])
    }
}
