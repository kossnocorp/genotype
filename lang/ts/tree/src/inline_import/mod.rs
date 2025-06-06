use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSInlineImport {
    pub path: TSPath,
    pub name: TSIdentifier,
}
