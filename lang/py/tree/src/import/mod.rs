use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYImport {
    pub dependency: PYDependencyIdent,
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
