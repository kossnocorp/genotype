use genotype_lang_py_config::{PYConfig, PYVersion};

use crate::{PYDependency, PYIdentifier};

use super::PYContext;

pub struct PYContextMock {
    imports: Vec<(PYDependency, PYIdentifier)>,

    config: PYConfig,
}

impl PYContextMock {
    pub fn new(version: PYVersion) -> Self {
        Self {
            imports: Vec::new(),
            config: PYConfig::new(version),
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
        Self::new(PYVersion::V3_12)
    }
}

impl PYContext for PYContextMock {
    fn import(&mut self, path: PYDependency, name: PYIdentifier) {
        self.imports.push((path, name));
    }

    fn is_version(&self, version: PYVersion) -> bool {
        self.config.version.clone().unwrap_or_default() == version
    }
}
