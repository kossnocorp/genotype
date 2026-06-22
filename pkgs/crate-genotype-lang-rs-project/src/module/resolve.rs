use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RsProjectModuleResolve {
    pub definitions: IndexMap<GtDefinitionId, GtpModuleResolveDefinition>,
}

impl RsProjectModuleResolve {
    pub fn new(module_resolve: &GtpModuleResolve) -> Self {
        Self {
            definitions: module_resolve.definitions.clone(),
        }
    }
}
