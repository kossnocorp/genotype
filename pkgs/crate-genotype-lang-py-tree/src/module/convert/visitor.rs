use crate::prelude::internal::*;

pub struct PyModuleVisitor {
    definitions: IndexSet<PyIdentifier>,
    scope: IndexSet<PyIdentifier>,
    current_alias: Option<PyIdentifier>,
    legacy: bool,
}

impl PyModuleVisitor {
    pub fn new(module: &PyModule, legacy: bool) -> Self {
        let definitions = module
            .definitions
            .iter()
            .map(|definition| definition.name().clone())
            .collect::<IndexSet<PyIdentifier>>();

        Self {
            definitions,
            scope: Default::default(),
            current_alias: None,
            legacy,
        }
    }
}

impl PyVisitor for PyModuleVisitor {}

impl PyVisitorMut for PyModuleVisitor {
    fn visit_definition_mut(&mut self, definition: &mut PyDefinition) {
        let name = definition.name().clone();
        self.scope.insert(name.clone());
        self.current_alias = if matches!(definition, PyDefinition::Alias(_)) {
            Some(name)
        } else {
            None
        };
    }

    fn visit_reference_mut(&mut self, reference: &mut PyReference) {
        if self.definitions.contains(&reference.identifier) {
            if self.legacy
                && self
                    .current_alias
                    .as_ref()
                    .is_some_and(|name| *name == reference.identifier)
            {
                return;
            }

            reference.forward = !self.scope.contains(&reference.identifier);
        }
    }
}
