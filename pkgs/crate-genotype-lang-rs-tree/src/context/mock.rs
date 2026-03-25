use crate::prelude::internal::*;

pub struct RsConvertContextMock {
    imports: Vec<(RsDependencyIdent, RsIdentifier)>,
}

impl RsConvertContextMock {
    pub fn with_imports(mut self, imports: Vec<(RsDependencyIdent, RsIdentifier)>) -> Self {
        self.imports = imports;
        self
    }

    pub fn as_imports(&self) -> &[(RsDependencyIdent, RsIdentifier)] {
        &self.imports
    }
}

impl RsConvertContextMockable for RsConvertContextMock {}

impl GtlConvertContext for RsConvertContextMock {
    type DependencyIdent = RsDependencyIdent;

    type DependencyRef = RsIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        self.imports.push((ident, r#ref));
    }
}

impl RsConvertContextConstraint for RsConvertContextMock {}

impl Default for RsConvertContextMock {
    fn default() -> Self {
        Self {
            imports: Vec::new(),
        }
    }
}
