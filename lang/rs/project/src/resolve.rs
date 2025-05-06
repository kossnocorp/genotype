use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct RSPModuleResolve {
    pub definitions: HashMap<GTDefinitionId, GTPModuleDefinitionResolve>,
}
