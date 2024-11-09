use crate::{identifier::RSIdentifier, path::RSPath};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSInlineUse {
    pub path: RSPath,
    pub name: RSIdentifier,
}
