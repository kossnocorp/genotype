use crate::GTSpan;

use super::{import_reference::GTImportReference, path::GTPath};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTImport {
    pub span: GTSpan,
    pub path: GTPath,
    pub reference: GTImportReference,
}
