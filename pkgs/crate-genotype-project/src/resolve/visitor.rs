use crate::prelude::internal::*;
use genotype_parser::visitor::GtVisitor;

pub struct GtpResolveVisitor<'a> {
    module_id: GtModuleId,
    resolve: &'a GtpResolve,
    error: Option<GtProjectError>,
    /// Module definitions resolve accumulated during the visit. It is then
    /// moved to corresponding module `GtpModuleResolve` struct.
    definitions: IndexMap<GtModuleId, IndexMap<GtDefinitionId, GtProjectModuleDefinitionResolve>>,
}

impl<'a> GtpResolveVisitor<'a> {
    pub fn new(module_id: GtModuleId, resolve: &'a GtpResolve) -> GtpResolveVisitor<'a> {
        GtpResolveVisitor {
            module_id,
            resolve,
            error: None,
            definitions: Default::default(),
        }
    }

    pub fn error(&self) -> Option<&GtProjectError> {
        self.error.as_ref()
    }

    pub fn drain_definitions(self) -> IndexMap<GtDefinitionId, GtProjectModuleDefinitionResolve> {
        self.definitions
            .get(&self.module_id)
            .and_then(|references| Some(references.clone()))
            .unwrap_or_default()
    }
}

impl GtVisitor for GtpResolveVisitor<'_> {}

impl GtVisitorMut for GtpResolveVisitor<'_> {
    fn visit_inline_import_mut(&mut self, import: &mut GtInlineImport) {
        if self.error.is_some() {
            return;
        }

        let Some(module_id) = self.resolve.path_module_ids.get(&import.path.id) else {
            return;
        };

        let Some(definitions) = self.resolve.definitions.get(module_id) else {
            return;
        };

        let is_defined = definitions
            .iter()
            .any(|definition| definition.1.as_ref() == import.name.as_str());
        if !is_defined {
            self.error = Some(GtProjectError::UndefinedType {
                span: import.name.as_span(),
                identifier: import.name.as_string(),
            });
        }
    }

    fn visit_reference_mut(&mut self, reference: &mut GtReference) {
        if self.error.is_some() {
            return;
        }

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
