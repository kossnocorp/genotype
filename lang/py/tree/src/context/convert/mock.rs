use crate::prelude::internal::*;

pub struct PYConvertContextMock {
    imports: Vec<(PYDependencyIdent, PYIdentifier)>,

    config: PyConfigLang,
}

impl PYConvertContextMock {
    pub fn new(version: PYVersion) -> Self {
        Self {
            imports: Vec::new(),
            config: PyConfigLang::new(version),
        }
    }

    pub fn with_imports(mut self, imports: Vec<(PYDependencyIdent, PYIdentifier)>) -> Self {
        self.imports = imports;
        self
    }

    pub fn as_imports(&self) -> &[(PYDependencyIdent, PYIdentifier)] {
        &self.imports
    }
}

impl PYConvertContextMockable for PYConvertContextMock {
    fn is_version(&self, version: PYVersion) -> bool {
        self.config.version == version
    }
}

impl GtlConvertContext for PYConvertContextMock {
    type DependencyIdent = PYDependencyIdent;

    type DependencyRef = PYIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        self.imports.push((ident, r#ref));
    }
}

impl PYConvertContextConstraint for PYConvertContextMock {}

impl Default for PYConvertContextMock {
    fn default() -> Self {
        Self::new(PYVersion::Latest)
    }
}
