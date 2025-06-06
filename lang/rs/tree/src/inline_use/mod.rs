use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSInlineUse {
    pub path: RSPath,
    pub name: RSIdentifier,
}
