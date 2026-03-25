use crate::prelude::internal::*;

pub struct PyConvertContextMock {
    imports: Vec<(PyDependencyIdent, PyIdentifier)>,

    config: PyConfigLang,
}

impl PyConvertContextMock {
    pub fn new(version: PyVersion) -> Self {
        Self {
            imports: Vec::new(),
            config: PyConfigLang::new(version),
        }
    }

    pub fn with_imports(mut self, imports: Vec<(PyDependencyIdent, PyIdentifier)>) -> Self {
        self.imports = imports;
        self
    }

    pub fn as_imports(&self) -> &[(PyDependencyIdent, PyIdentifier)] {
        &self.imports
    }
}

impl PyConvertContextMockable for PyConvertContextMock {
    fn is_version(&self, version: PyVersion) -> bool {
        self.config.version == version
    }
}

impl GtlConvertContext for PyConvertContextMock {
    type DependencyIdent = PyDependencyIdent;

    type DependencyRef = PyIdentifier;

    fn add_import(self: &mut Self, ident: Self::DependencyIdent, r#ref: Self::DependencyRef) {
        self.imports.push((ident, r#ref));
    }
}

impl PyConvertContextConstraint for PyConvertContextMock {}

impl Default for PyConvertContextMock {
    fn default() -> Self {
        Self::new(PyVersion::Latest)
    }
}
