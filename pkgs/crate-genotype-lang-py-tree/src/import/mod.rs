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
        Self {
            dependency,
            reference,
        }
    }
}

impl GtlImport for PyImport {
    type DependencyIdent = PyDependencyIdent;
    type ImportRef = PyImportReference;
    type ImportRefName = PyImportName;

    fn dependency(&self) -> &Self::DependencyIdent {
        &self.dependency
    }

    fn reference(&self) -> &Self::ImportRef {
        &self.reference
    }

    fn ref_names(&self) -> Option<&Vec<Self::ImportRefName>> {
        match &self.reference {
            PyImportReference::Named(idents) => Some(idents),
            _ => None,
        }
    }

    fn ref_names_mut(&mut self) -> Option<&mut Vec<Self::ImportRefName>> {
        match &mut self.reference {
            PyImportReference::Named(idents) => Some(idents),
            _ => None,
        }
    }
}
