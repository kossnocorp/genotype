use crate::import_reference::TSImportReference;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSImport {
    pub path: String,
    pub reference: TSImportReference,
}
