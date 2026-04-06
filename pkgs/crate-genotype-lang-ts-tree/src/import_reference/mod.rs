use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsImportReference {
    Default(Arc<str>),
    Glob(Arc<str>),
    Named(#[visit] Vec<TsImportName>),
}

impl GtlImportRef for TsImportReference {}

impl From<&str> for TsImportReference {
    fn from(str: &str) -> Self {
        TsImportReference::Named(vec![str.into()])
    }
}

impl From<TsIdentifier> for TsImportReference {
    fn from(identifier: TsIdentifier) -> Self {
        TsImportReference::Named(vec![TsImportName::Name(identifier)])
    }
}
