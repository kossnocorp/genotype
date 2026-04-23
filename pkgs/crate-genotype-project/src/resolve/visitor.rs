use crate::prelude::internal::*;
use genotype_parser::visitor::GtVisitor;

pub struct GtpResolveVisitor<'a> {
    module_id: GtModuleId,
    resolve: &'a GtpResolve,
    error: Option<GtpError>,
    /// Module definitions resolve accumulated during the visit. It is then
    /// moved to corresponding module `GtpModuleResolve` struct.
    definitions: IndexMap<GtModuleId, IndexMap<GtDefinitionId, GtpModuleDefinitionResolve>>,
    /// Reference definition resolve accumulated during the visit.
    reference_definition_ids: IndexMap<GtReferenceId, GtDefinitionId>,
}

impl<'a> GtpResolveVisitor<'a> {
    pub fn new(module_id: GtModuleId, resolve: &'a GtpResolve) -> GtpResolveVisitor<'a> {
        GtpResolveVisitor {
            module_id,
            resolve,
            error: None,
            definitions: Default::default(),
            reference_definition_ids: Default::default(),
        }
    }

    pub fn error(&self) -> Option<&GtpError> {
        self.error.as_ref()
    }

    pub fn drain_definitions(&self) -> IndexMap<GtDefinitionId, GtpModuleDefinitionResolve> {
        self.definitions
            .get(&self.module_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_reference_definition_ids(&self) -> IndexMap<GtReferenceId, GtDefinitionId> {
        self.reference_definition_ids.clone()
    }
}

impl GtVisitor for GtpResolveVisitor<'_> {
    fn visit_inline_import(&mut self, import: &GtInlineImport) {
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
            self.error = Some(GtpError::UndefinedType {
                span: import.name.as_span(),
                identifier: import.name.as_string(),
            });
        }
    }

    fn visit_reference(&mut self, reference: &GtReference) {
        if self.error.is_some() {
            return;
        }

        let resolved_definition_id =
            if let Some(definitions) = self.resolve.definitions.get(&self.module_id) {
                definitions
                    .iter()
                    .find(|definition| definition.1 == reference.identifier.1)
                    .cloned()
            } else {
                None
            }
            .or_else(|| {
                self.resolve
                    .imports
                    .get(&self.module_id)
                    .and_then(|definitions| {
                        definitions
                            .iter()
                            .find(|definition| definition.1 == reference.identifier.1)
                            .cloned()
                    })
            });

        if let Some(definition_id) = resolved_definition_id {
            self.reference_definition_ids
                .insert(reference.id.clone(), definition_id.clone());

            let resolve = self
                .definitions
                .entry(self.module_id.clone())
                .or_default()
                .entry(definition_id)
                .or_default();
            resolve.references.insert(reference.id.clone());
        }

        // [TODO] Make visitor return results, so we can handle unresolved references
    }
}
