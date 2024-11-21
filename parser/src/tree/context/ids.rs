use crate::{GTDefinitionId, GTIdentifier};

use super::GTContext;

impl GTContext {
    pub fn get_definition_id(&self, name: &GTIdentifier) -> GTDefinitionId {
        GTDefinitionId(self.module_id.clone(), name.1.clone())
    }
}
