use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSImport {
    pub path: TSPath,
    pub reference: TSImportReference,
}

impl TSImport {
    pub fn new(path: TSPath, reference: TSImportReference) -> Self {
        TSImport { path, reference }
    }
}

impl GtlImport for TSImport {}
