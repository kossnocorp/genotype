use super::{import_reference::GTImportReference, path::GTPath};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTImport {
    pub path: GTPath,
    pub reference: GTImportReference,
}
