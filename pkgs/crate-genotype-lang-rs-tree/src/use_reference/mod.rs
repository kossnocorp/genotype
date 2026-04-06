use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum RsUseReference {
    /// Module use, i.e. `use super::collection;`
    Module,
    /// Glob use, i.e. `use super::collection::*;`
    Glob,
    /// Named use, i.e. `use super::collection::{Collection, CollectionItem};`
    Named(#[visit] Vec<RsUseName>),
}

impl GtlImportRef for RsUseReference {}

impl From<&str> for RsUseReference {
    fn from(str: &str) -> Self {
        RsUseReference::Named(vec![str.into()])
    }
}

impl From<RsIdentifier> for RsUseReference {
    fn from(identifier: RsIdentifier) -> Self {
        RsUseReference::Named(vec![identifier.into()])
    }
}
