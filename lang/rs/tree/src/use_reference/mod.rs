use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSUseReference {
    Module,
    Glob,
    Named(Vec<RSUseName>),
}
