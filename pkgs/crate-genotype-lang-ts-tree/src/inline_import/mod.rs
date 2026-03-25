use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsInlineImport {
    #[visit]
    pub path: TsPath,
    #[visit]
    pub name: TsIdentifier,
}
