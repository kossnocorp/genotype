use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTInlineImport {
    pub span: GTSpan,
    pub name: GTIdentifier,
    pub path: GTPath,
}
