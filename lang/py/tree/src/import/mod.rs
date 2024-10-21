use crate::{import_reference::PYImportReference, path::PYPath, PYDependency};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYImport {
    // [TODO] Probably can get rid of it in favor of dependency
    pub path: PYPath,
    pub reference: PYImportReference,
    pub dependency: PYDependency,
}
