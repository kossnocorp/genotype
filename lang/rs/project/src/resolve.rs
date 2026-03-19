use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RSPModuleResolve {
    pub definitions: IndexMap<GTDefinitionId, GtProjectModuleDefinitionResolve>,
}
