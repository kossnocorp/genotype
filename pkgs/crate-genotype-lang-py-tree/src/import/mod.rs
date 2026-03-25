use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyImport {
    #[visit]
    pub dependency: PyDependencyIdent,
    #[visit]
    pub reference: PyImportReference,
}

impl PyImport {
    pub fn new(dependency: PyDependencyIdent, reference: PyImportReference) -> Self {
        PyImport {
            dependency,
            reference,
        }
    }
}

impl GtlImport for PyImport {}
