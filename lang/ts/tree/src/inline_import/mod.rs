use crate::{identifier::TSIdentifier, path::TSPath};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSInlineImport {
    pub path: TSPath,
    pub name: TSIdentifier,
}
