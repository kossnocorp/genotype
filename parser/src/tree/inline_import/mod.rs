use super::{identifier::GTIdentifier, path::GTPath};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTInlineImport {
    pub path: GTPath,
    pub name: GTIdentifier,
}
