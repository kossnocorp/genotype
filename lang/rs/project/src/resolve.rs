use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RSPModuleResolve {
    pub definitions: HashMap<GTDefinitionId, GtProjectModuleDefinitionResolve>,
}
