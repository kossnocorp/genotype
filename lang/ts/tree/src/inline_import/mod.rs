use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TSInlineImport {
    pub path: TSPath,
    pub name: TSIdentifier,
}
