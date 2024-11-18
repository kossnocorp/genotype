use genotype_lang_rs_tree::RSIdentifier;
use genotype_parser::GTAliasId;

use super::RSConvertContext;

impl RSConvertContext {
    pub fn provide_alias_id(&mut self, doc: GTAliasId) {
        self.alias_id = Some(doc);
    }

    pub fn drop_alias_id(&mut self) {
        self.consume_alias_id();
    }

    pub fn consume_alias_id(&mut self) -> Option<GTAliasId> {
        self.alias_id.take()
    }

    pub fn build_alias_id(&self, identifier: &RSIdentifier) -> GTAliasId {
        GTAliasId(self.module_id.clone(), identifier.0.clone())
    }
}
