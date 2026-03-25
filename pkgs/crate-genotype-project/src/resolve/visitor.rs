use crate::prelude::internal::*;
use genotype_parser::visitor::GtVisitor;

pub struct GtpResolveVisitor<'a> {
    module_id: GtModuleId,
    resolve: &'a GtpResolve,
    /// Module definitions resolve accumulated during the visit. It is then
    /// moved to corresponding module `GtpModuleResolve` struct.
    definitions: IndexMap<GtModuleId, IndexMap<GtDefinitionId, GtProjectModuleDefinitionResolve>>,
}

impl<'a> GtpResolveVisitor<'a> {
    pub fn new(module_id: GtModuleId, resolve: &'a GtpResolve) -> GtpResolveVisitor<'a> {
        GtpResolveVisitor {
            module_id,
            resolve,
            definitions: Default::default(),
        }
    }

    pub fn drain_definitions(self) -> IndexMap<GtDefinitionId, GtProjectModuleDefinitionResolve> {
        self.definitions
            .get(&self.module_id)
            .and_then(|references| Some(references.clone()))
            .unwrap_or_default()
    }
}

impl GtVisitor for GtpResolveVisitor<'_> {
    fn visit_import(&mut self, import: &mut GtImport) {
        if let GtPathModuleId::Unresolved = &import.path.1 {
            let module_paths = self.resolve.paths.get(&self.module_id).unwrap();
            let module_id = module_paths.get(import.path.source_str()).unwrap();
            import.path.1 = GtPathModuleId::Resolved(module_id.clone());
        }
    }

    fn visit_inline_import(&mut self, import: &mut GtInlineImport) {
        if let GtPathModuleId::Unresolved = &import.path.1 {
            let module_paths = self.resolve.paths.get(&self.module_id).unwrap();
            let module_id = module_paths.get(import.path.source_str()).unwrap();
            import.path.1 = GtPathModuleId::Resolved(module_id.clone());
        }
    }

    fn visit_reference(&mut self, reference: &mut GtReference) {
        match &reference.definition_id {
            GtReferenceDefinitionId::Unresolved => {
                if let Some(definitions) = self.resolve.definitions.get(&self.module_id) {
                    let definition = definitions
                        .iter()
                        .find(|definition| definition.1 == reference.identifier.1);
                    if let Some(local_definition) = definition {
                        reference.definition_id =
                            GtReferenceDefinitionId::Resolved(local_definition.clone());

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
                            GtReferenceDefinitionId::Resolved(imported_definition.clone());

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

            GtReferenceDefinitionId::Resolved(definition_id) => {
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
