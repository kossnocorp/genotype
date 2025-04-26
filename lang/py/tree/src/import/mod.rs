use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYImport {
    pub reference: PYImportReference,
    pub dependency: PYDependencyIdent,
}

impl GtlImport for PYImport {}
