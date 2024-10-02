use crate::{import_reference::TSImportReference, path::TSPath};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSImport {
    pub path: TSPath,
    pub reference: TSImportReference,
}
