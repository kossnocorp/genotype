use genotype_lang_py_config::{PYLangConfig, PYVersion};

use crate::{PYDependency, PYIdentifier};

use super::PYContext;

pub struct PYContextMock {
    imports: Vec<(PYDependency, PYIdentifier)>,

    config: PYLangConfig,
}

impl PYContextMock {
    pub fn new(version: PYVersion) -> Self {
        Self {
            imports: Vec::new(),
            config: PYLangConfig::new(version),
        }
    }

    pub fn with_imports(mut self, imports: Vec<(PYDependency, PYIdentifier)>) -> Self {
        self.imports = imports;
        self
    }

    pub fn as_imports(&self) -> &[(PYDependency, PYIdentifier)] {
        &self.imports
    }
}

impl Default for PYContextMock {
    fn default() -> Self {
        Self::new(PYVersion::Latest)
    }
}

impl PYContext for PYContextMock {
    fn import(&mut self, path: PYDependency, name: PYIdentifier) {
        self.imports.push((path, name));
    }

    fn is_version(&self, version: PYVersion) -> bool {
        self.config.version == version
    }
}
