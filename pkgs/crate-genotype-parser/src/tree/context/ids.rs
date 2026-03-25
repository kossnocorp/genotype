use crate::prelude::internal::*;

impl GtContext {
    pub fn get_definition_id(&self, name: &GtIdentifier) -> GtDefinitionId {
        GtDefinitionId(self.module_id.clone(), name.1.clone())
    }
}
