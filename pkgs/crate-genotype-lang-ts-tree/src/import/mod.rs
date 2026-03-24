use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSImport {
    #[visit]
    pub path: TSPath,
    #[visit]
    pub reference: TSImportReference,
}

impl TSImport {
    pub fn new(path: TSPath, reference: TSImportReference) -> Self {
        TSImport { path, reference }
    }
}

impl GtlImport for TSImport {}
