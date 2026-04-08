use crate::prelude::internal::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct RsConvertResolve {
    pub paths: HashMap<GtPath, GtPath>,
    pub path_module_ids: HashMap<GtPathModuleId, GtModuleId>,
    pub reference_definition_ids: HashMap<GtReferenceId, GtDefinitionId>,
    pub globs: HashMap<GtPath, String>,
    pub identifiers: HashMap<GtIdentifier, GtIdentifier>,
    pub imported: HashSet<GtIdentifier>,
}
