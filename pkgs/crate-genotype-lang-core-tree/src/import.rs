use crate::prelude::internal::*;

pub trait GtlImport {
    type DependencyIdent: GtlDependencyIdent;
    type ImportRef: GtlImportRef;
    type ImportRefName: GtlImportRefName;

    fn dependency(&self) -> &Self::DependencyIdent;

    fn reference(&self) -> &Self::ImportRef;

    fn ref_names(&self) -> Option<&Vec<Self::ImportRefName>>;

    fn ref_names_mut(&mut self) -> Option<&mut Vec<Self::ImportRefName>>;
}

pub trait GtlImportRef: PartialEq {}

pub trait GtlImportRefName: Clone + PartialEq {}
