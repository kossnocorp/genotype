use crate::{import_reference::RSImportReference, path::RSPath, RSDependency};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSImport {
    // [TODO] Probably can get rid of it in favor of dependency
    pub path: RSPath,
    pub reference: RSImportReference,
    pub dependency: RSDependency,
}
