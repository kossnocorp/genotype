use genotype_lang_rs_tree::RSIdentifier;
use genotype_parser::GTDefinitionId;

use super::RSConvertContext;

impl RSConvertContext {
    pub fn provide_definition_id(&mut self, doc: GTDefinitionId) {
        self.definition_id = Some(doc);
    }

    pub fn drop_definition_id(&mut self) {
        self.consume_definition_id();
    }

    pub fn consume_definition_id(&mut self) -> Option<GTDefinitionId> {
        self.definition_id.take()
    }

    pub fn build_definition_id(&self, identifier: &RSIdentifier) -> GTDefinitionId {
        GTDefinitionId(self.module_id.clone(), identifier.0.clone())
    }
}
