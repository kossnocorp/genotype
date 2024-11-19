use std::collections::{HashMap, HashSet};

use genotype_parser::*;
use genotype_visitor::visitor::GTVisitor;

use super::GTProjectResolve;

pub struct GTProjectResolveVisitor<'a> {
    module_id: GTModuleId,
    resolve: &'a GTProjectResolve,
    /// Map of references for each module. It serves as a reference counter.
    references: HashMap<GTModuleId, HashMap<GTDefinitionId, HashSet<GTSpan>>>,
}

impl<'a> GTProjectResolveVisitor<'a> {
    pub fn new(
        module_id: GTModuleId,
        resolve: &'a GTProjectResolve,
    ) -> GTProjectResolveVisitor<'a> {
        GTProjectResolveVisitor {
            module_id,
            resolve,
            references: Default::default(),
        }
    }

    pub fn drain_references(self) -> HashMap<GTDefinitionId, HashSet<GTSpan>> {
        self.references
            .get(&self.module_id)
            .and_then(|references| Some(references.clone()))
            .unwrap_or_default()
    }
}

impl GTVisitor for GTProjectResolveVisitor<'_> {
    fn visit_import(&mut self, import: &mut GTImport) {
        if let GTPathModuleId::Unresolved = &import.path.1 {
            let module_paths = self.resolve.paths.get(&self.module_id).unwrap();
            let module_id = module_paths.get(import.path.as_str()).unwrap();
            import.path.1 = GTPathModuleId::Resolved(module_id.clone());
        }
    }

    fn visit_reference(&mut self, reference: &mut GTReference) {
        match &reference.1 {
            GTReferenceDefinitionId::Unresolved => {
                if let Some(definitions) = self.resolve.definitions.get(&self.module_id) {
                    let definition = definitions
                        .iter()
                        .find(|definition| definition.1 == reference.2 .1);
                    if let Some(local_definition) = definition {
                        reference.1 = GTReferenceDefinitionId::Resolved(local_definition.clone());

                        self.references
                            .entry(self.module_id.clone())
                            .or_default()
                            .entry(local_definition.clone())
                            .or_default()
                            .insert(reference.0.clone());
                    }
                }

                if let Some(imported) = self.resolve.imports.get(&self.module_id) {
                    let definition = imported
                        .iter()
                        .find(|definition| definition.1 == reference.2 .1);
                    if let Some(imported_definition) = definition {
                        reference.1 =
                            GTReferenceDefinitionId::Resolved(imported_definition.clone());

                        self.references
                            .entry(self.module_id.clone())
                            .or_default()
                            .entry(imported_definition.clone())
                            .or_default()
                            .insert(reference.0.clone());
                    }
                }

                // [TODO] Make visitor return results, so we can handle unresolved references
            }

            GTReferenceDefinitionId::Resolved(definition_id) => {
                self.references
                    .entry(self.module_id.clone())
                    .or_default()
                    .entry(definition_id.clone())
                    .or_default()
                    .insert(reference.0.clone());
            }
        }
    }
}
