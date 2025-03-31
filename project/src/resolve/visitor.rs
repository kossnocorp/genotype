use std::collections::{HashMap, HashSet};

use genotype_parser::*;
use genotype_visitor::visitor::GTVisitor;

use crate::GTPModuleDefinitionResolve;

use super::GTPResolve;

pub struct GTPResolveVisitor<'a> {
    module_id: GTModuleId,
    resolve: &'a GTPResolve,
    /// Module definitions resolve accumulated during the visit. It is then
    /// moved to corresponding module `GTPModuleResolve` struct.
    definitions: HashMap<GTModuleId, HashMap<GTDefinitionId, GTPModuleDefinitionResolve>>,
}

impl<'a> GTPResolveVisitor<'a> {
    pub fn new(module_id: GTModuleId, resolve: &'a GTPResolve) -> GTPResolveVisitor<'a> {
        GTPResolveVisitor {
            module_id,
            resolve,
            definitions: Default::default(),
        }
    }

    pub fn drain_definitions(self) -> HashMap<GTDefinitionId, GTPModuleDefinitionResolve> {
        self.definitions
            .get(&self.module_id)
            .and_then(|references| Some(references.clone()))
            .unwrap_or_default()
    }
}

impl GTVisitor for GTPResolveVisitor<'_> {
    fn visit_import(&mut self, import: &mut GTImport) {
        if let GTPathModuleId::Unresolved = &import.path.1 {
            let module_paths = self.resolve.paths.get(&self.module_id).unwrap();
            let module_id = module_paths.get(import.path.as_str()).unwrap();
            import.path.1 = GTPathModuleId::Resolved(module_id.clone());
        }
    }

    fn visit_inline_import(&mut self, import: &mut GTInlineImport) {
        if let GTPathModuleId::Unresolved = &import.path.1 {
            let module_paths = self.resolve.paths.get(&self.module_id).unwrap();
            let module_id = module_paths.get(import.path.as_str()).unwrap();
            import.path.1 = GTPathModuleId::Resolved(module_id.clone());
        }
    }

    fn visit_reference(&mut self, reference: &mut GTReference) {
        match &reference.definition_id {
            GTReferenceDefinitionId::Unresolved => {
                if let Some(definitions) = self.resolve.definitions.get(&self.module_id) {
                    let definition = definitions
                        .iter()
                        .find(|definition| definition.1 == reference.identifier.1);
                    if let Some(local_definition) = definition {
                        reference.definition_id =
                            GTReferenceDefinitionId::Resolved(local_definition.clone());

                        let resolve = self
                            .definitions
                            .entry(self.module_id.clone())
                            .or_default()
                            .entry(local_definition.clone())
                            .or_default();
                        resolve.references.insert(reference.id.clone());
                    }
                }

                if let Some(imported) = self.resolve.imports.get(&self.module_id) {
                    let definition = imported
                        .iter()
                        .find(|definition| definition.1 == reference.identifier.1);
                    if let Some(imported_definition) = definition {
                        reference.definition_id =
                            GTReferenceDefinitionId::Resolved(imported_definition.clone());

                        let resolve = self
                            .definitions
                            .entry(self.module_id.clone())
                            .or_default()
                            .entry(imported_definition.clone())
                            .or_default();
                        resolve.references.insert(reference.id.clone());
                    }
                }

                // [TODO] Make visitor return results, so we can handle unresolved references
            }

            GTReferenceDefinitionId::Resolved(definition_id) => {
                let resolve = self
                    .definitions
                    .entry(self.module_id.clone())
                    .or_default()
                    .entry(definition_id.clone())
                    .or_default();
                resolve.references.insert(reference.id.clone());
            }
        }
    }
}
