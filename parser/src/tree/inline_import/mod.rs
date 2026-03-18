use serde::Serialize;

use crate::GTSpan;

use super::{identifier::GTIdentifier, path::GTPath};

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTInlineImport {
    pub span: GTSpan,
    pub name: GTIdentifier,
    pub path: GTPath,
}
