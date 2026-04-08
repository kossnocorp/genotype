use crate::prelude::internal::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub struct RsConvertResolve {
    pub paths: HashMap<GtPath, GtPath>,
    pub path_module_ids: HashMap<GtPathModuleId, GtModuleId>,
    pub reference_definition_ids: HashMap<GtReferenceId, GtDefinitionId>,
    pub globs: HashMap<GtPath, String>,
    pub identifiers: HashMap<GtIdentifier, GtIdentifier>,
    pub imported: HashSet<GtIdentifier>,
}

impl Default for RsConvertResolve {
    fn default() -> Self {
        Self {
            paths: HashMap::new(),
            path_module_ids: HashMap::new(),
            reference_definition_ids: HashMap::new(),
            globs: HashMap::new(),
            identifiers: HashMap::new(),
            imported: HashSet::new(),
        }
    }
}
