use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct RsConvertResolve {
    pub paths: IndexMap<GtPath, GtPath>,
    pub path_module_ids: IndexMap<GtPathModuleId, GtModuleId>,
    pub reference_definition_ids: IndexMap<GtReferenceId, GtDefinitionId>,
    pub globs: IndexMap<GtPath, String>,
    pub identifiers: IndexMap<GtIdentifier, GtIdentifier>,
    pub imported: IndexSet<GtIdentifier>,
}
