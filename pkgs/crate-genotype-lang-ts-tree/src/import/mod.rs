use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsImport {
    #[visit]
    pub dependency: TsDependencyIdent,
    #[visit]
    pub reference: TsImportReference,
}

impl TsImport {
    pub fn new(dependency: TsDependencyIdent, reference: TsImportReference) -> Self {
        Self {
            dependency,
            reference,
        }
    }
}

impl GtlImport for TsImport {
    type DependencyIdent = TsDependencyIdent;
    type ImportRef = TsImportReference;
    type ImportRefName = TsImportName;

    fn dependency(&self) -> &Self::DependencyIdent {
        &self.dependency
    }

    fn reference(&self) -> &Self::ImportRef {
        &self.reference
    }

    fn ref_names(&self) -> Option<&Vec<Self::ImportRefName>> {
        match &self.reference {
            TsImportReference::Named(idents) => Some(idents),
            _ => None,
        }
    }

    fn ref_names_mut(&mut self) -> Option<&mut Vec<Self::ImportRefName>> {
        match &mut self.reference {
            TsImportReference::Named(idents) => Some(idents),
            _ => None,
        }
    }
}
