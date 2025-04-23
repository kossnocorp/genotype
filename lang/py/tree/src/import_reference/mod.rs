use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYImportReference {
    Default(Option<PYIdentifier>),
    Glob,
    Named(Vec<PYImportName>),
}
