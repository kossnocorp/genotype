use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSUseReference {
    /// Module use, i.e. `use super::collection;`
    Module,
    /// Glob use, i.e. `use super::collection::*;`
    Glob,
    /// Named use, i.e. `use super::collection::{Collection, CollectionItem};`
    Named(Vec<RSUseName>),
}
