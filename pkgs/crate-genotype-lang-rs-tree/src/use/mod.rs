use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsUse {
    #[visit]
    pub dependency: RsDependencyIdent,
    #[visit]
    pub reference: RsUseReference,
}

impl RsUse {
    pub fn new(dependency: RsDependencyIdent, reference: RsUseReference) -> Self {
        Self {
            dependency,
            reference,
        }
    }
}

impl GtlImport for RsUse {
    type DependencyIdent = RsDependencyIdent;
    type ImportRef = RsUseReference;
    type ImportRefName = RsUseName;

    fn dependency(&self) -> &Self::DependencyIdent {
        &self.dependency
    }

    fn reference(&self) -> &Self::ImportRef {
        &self.reference
    }

    fn ref_names(&self) -> Option<&Vec<Self::ImportRefName>> {
        match &self.reference {
            RsUseReference::Named(idents) => Some(idents),
            _ => None,
        }
    }

    fn ref_names_mut(&mut self) -> Option<&mut Vec<Self::ImportRefName>> {
        match &mut self.reference {
            RsUseReference::Named(idents) => Some(idents),
            _ => None,
        }
    }
}
