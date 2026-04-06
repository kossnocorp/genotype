use crate::prelude::internal::*;
use std::collections::HashSet;

pub struct PyModuleVisitor {
    definitions: HashSet<PyIdentifier>,
    scope: HashSet<PyIdentifier>,
}

impl PyModuleVisitor {
    pub fn new(module: &PyModule) -> Self {
        let definitions = module
            .definitions
            .iter()
            .map(|definition| definition.name().clone())
            .collect::<HashSet<PyIdentifier>>();

        Self {
            definitions,
            scope: Default::default(),
        }
    }
}

impl PyVisitor for PyModuleVisitor {
    fn visit_definition(&mut self, definition: &PyDefinition) {
        self.scope.insert(definition.name().clone());
    }
}

impl PyVisitorMut for PyModuleVisitor {
    fn visit_reference_mut(&mut self, reference: &mut PyReference) {
        if self.definitions.contains(&reference.identifier) {
            reference.forward = !self.scope.contains(&reference.identifier);
        }
    }
}
