use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYImport {
    #[visit]
    pub dependency: PYDependencyIdent,
    #[visit]
    pub reference: PYImportReference,
}

impl PYImport {
    pub fn new(dependency: PYDependencyIdent, reference: PYImportReference) -> Self {
        PYImport {
            dependency,
            reference,
        }
    }
}

impl GtlImport for PYImport {}
