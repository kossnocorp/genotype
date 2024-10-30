use std::collections::HashSet;

use genotype_lang_py_tree::*;
use genotype_lang_py_visitor::visitor::PYVisitor;

pub struct PYModuleVisitor {
    definitions: HashSet<PYIdentifier>,
    scope: HashSet<PYIdentifier>,
}

impl PYModuleVisitor {
    pub fn new(module: &PYModule) -> Self {
        let definitions = module
            .definitions
            .iter()
            .map(|definition| definition.name().clone())
            .collect::<HashSet<PYIdentifier>>();

        Self {
            definitions,
            scope: Default::default(),
        }
    }
}

impl PYVisitor for PYModuleVisitor {
    fn visit_definition(&mut self, definition: &mut PYDefinition) {
        self.scope.insert(definition.name().clone());
    }

    fn visit_reference(&mut self, reference: &mut PYReference) {
        if self.definitions.contains(&reference.identifier) {
            reference.forward = !self.scope.contains(&reference.identifier);
            self.scope.insert(reference.identifier.clone());
        }
    }
}
