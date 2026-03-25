use crate::prelude::internal::*;

pub struct TsConvertContextMock {
    imports: Vec<(TsDependencyIdent, TsIdentifier)>,
}

impl TsConvertContextMock {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }

    pub fn with_imports(mut self, imports: Vec<(TsDependencyIdent, TsIdentifier)>) -> Self {
        self.imports = imports;
        self
    }

    pub fn as_imports(&self) -> &[(TsDependencyIdent, TsIdentifier)] {
        &self.imports
    }
}

impl TsConvertContextMockable for TsConvertContextMock {}

impl GtlConvertContext for TsConvertContextMock {
    type DependencyIdent = TsDependencyIdent;

    type DependencyRef = TsIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        self.imports.push((ident, r#ref));
    }
}

impl TsConvertContextConstraint for TsConvertContextMock {}
