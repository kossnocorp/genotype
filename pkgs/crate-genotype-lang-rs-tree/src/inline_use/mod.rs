use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsInlineUse {
    #[visit]
    pub path: RsPath,
    #[visit]
    pub name: RsIdentifier,
}
