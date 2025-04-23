use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSImport {
    pub path: TSPath,
    pub reference: TSImportReference,
}
