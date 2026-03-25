use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RspModuleResolve {
    pub definitions: IndexMap<GtDefinitionId, GtProjectModuleDefinitionResolve>,
}
