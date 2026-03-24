use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSInlineUse {
    #[visit]
    pub path: RSPath,
    #[visit]
    pub name: RSIdentifier,
}
