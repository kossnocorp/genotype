use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSInlineImport {
    #[visit]
    pub path: TSPath,
    #[visit]
    pub name: TSIdentifier,
}
