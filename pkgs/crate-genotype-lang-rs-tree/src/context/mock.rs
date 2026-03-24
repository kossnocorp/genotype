use crate::prelude::internal::*;

pub struct RSConvertContextMock {
    imports: Vec<(RSDependencyIdent, RSIdentifier)>,
}

impl RSConvertContextMock {
    pub fn with_imports(mut self, imports: Vec<(RSDependencyIdent, RSIdentifier)>) -> Self {
        self.imports = imports;
        self
    }

    pub fn as_imports(&self) -> &[(RSDependencyIdent, RSIdentifier)] {
        &self.imports
    }
}

impl RSConvertContextMockable for RSConvertContextMock {}

impl GtlConvertContext for RSConvertContextMock {
    type DependencyIdent = RSDependencyIdent;

    type DependencyRef = RSIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        self.imports.push((ident, r#ref));
    }
}

impl RSConvertContextConstraint for RSConvertContextMock {}

impl Default for RSConvertContextMock {
    fn default() -> Self {
        Self {
            imports: Vec::new(),
        }
    }
}
