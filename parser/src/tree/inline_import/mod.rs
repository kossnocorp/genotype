use super::{identifier::GTIdentifier, path::GTPath};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTInlineImport {
    pub name: GTIdentifier,
    pub path: GTPath,
}
