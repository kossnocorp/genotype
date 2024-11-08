use genotype_lang_rs_config::{RSLangConfig, RSVersion};

use crate::{RSDependency, RSIdentifier};

use super::RSContext;

pub struct RSContextMock {
    imports: Vec<(RSDependency, RSIdentifier)>,

    config: RSLangConfig,
}

impl RSContextMock {
    pub fn new(version: RSVersion) -> Self {
        Self {
            imports: Vec::new(),
            config: RSLangConfig::new(version),
        }
    }

    pub fn with_imports(mut self, imports: Vec<(RSDependency, RSIdentifier)>) -> Self {
        self.imports = imports;
        self
    }

    pub fn as_imports(&self) -> &[(RSDependency, RSIdentifier)] {
        &self.imports
    }
}

impl Default for RSContextMock {
    fn default() -> Self {
        Self::new(RSVersion::Latest)
    }
}

impl RSContext for RSContextMock {
    fn import(&mut self, path: RSDependency, name: RSIdentifier) {
        self.imports.push((path, name));
    }

    fn is_version(&self, version: RSVersion) -> bool {
        self.config.version == version
    }
}
