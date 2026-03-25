use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsImport {
    #[visit]
    pub path: TsPath,
    #[visit]
    pub reference: TsImportReference,
}

impl TsImport {
    pub fn new(path: TsPath, reference: TsImportReference) -> Self {
        TsImport { path, reference }
    }
}

impl GtlImport for TsImport {}
