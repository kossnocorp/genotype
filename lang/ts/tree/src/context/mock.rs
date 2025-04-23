use crate::prelude::internal::*;

pub struct TSConvertContextMock {
    imports: Vec<(TSDependencyIdent, TSIdentifier)>,
}

impl TSConvertContextMock {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }

    pub fn with_imports(mut self, imports: Vec<(TSDependencyIdent, TSIdentifier)>) -> Self {
        self.imports = imports;
        self
    }

    pub fn as_imports(&self) -> &[(TSDependencyIdent, TSIdentifier)] {
        &self.imports
    }
}

impl TSConvertContextMockable for TSConvertContextMock {}

impl GtlConvertContext for TSConvertContextMock {
    type DependencyIdent = TSDependencyIdent;

    type DependencyRef = TSIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        self.imports.push((ident, r#ref));
    }
}

impl TSConvertContextConstraint for TSConvertContextMock {}
