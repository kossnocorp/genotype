use crate::{RSDependency, RSIdentifier};

use super::RSContext;

pub struct RSContextMock {
    imports: Vec<(RSDependency, RSIdentifier)>,
}

impl RSContextMock {
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
        Self {
            imports: Vec::new(),
        }
    }
}

impl RSContext for RSContextMock {
    fn import(&mut self, path: RSDependency, name: RSIdentifier) {
        self.imports.push((path, name));
    }
}
