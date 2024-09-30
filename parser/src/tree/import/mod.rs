use super::import_reference::GTImportReference;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTImport {
    pub path: String,
    pub reference: GTImportReference,
}
