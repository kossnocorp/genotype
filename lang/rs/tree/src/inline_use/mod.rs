use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSInlineUse {
    pub path: RSPath,
    pub name: RSIdentifier,
}
