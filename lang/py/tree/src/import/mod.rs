use crate::{import_reference::PYImportReference, path::PYPath};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYImport {
    pub path: PYPath,
    pub reference: PYImportReference,
}
