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
