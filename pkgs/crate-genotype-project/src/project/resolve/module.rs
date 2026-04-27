use crate::prelude::internal::*;

#[derive(Default)]
pub struct GtpResolveModule {
    /// Module definitions. The definitions can be either root, nested or synthetic
    /// (where the name is derived from the parents). It defines what is exported from the module.
    pub definitions: Vec<GtDefinitionId>,
    /// Module imports. It defines what is imported from the other modules.
    pub imports: Vec<GtDefinitionId>,
    /// Map of local path to module id. It allows to quickly resolve the module id
    /// from any local path.
    pub paths: IndexMap<String, GtModuleId>,
}
