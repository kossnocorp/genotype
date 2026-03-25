use crate::prelude::internal::*;

impl RsConvertContext {
    pub fn provide_definition_id(&mut self, doc: GtDefinitionId) {
        self.definition_id = Some(doc);
    }

    pub fn drop_definition_id(&mut self) {
        self.consume_definition_id();
    }

    pub fn consume_definition_id(&mut self) -> Option<GtDefinitionId> {
        self.definition_id.take()
    }

    pub fn build_definition_id(&self, identifier: &RsIdentifier) -> GtDefinitionId {
        GtDefinitionId(self.module_id.clone(), identifier.0.clone())
    }

    pub fn reference_id(&self, span: GtSpan) -> GtReferenceId {
        GtReferenceId(self.module_id.clone(), span)
    }
}
