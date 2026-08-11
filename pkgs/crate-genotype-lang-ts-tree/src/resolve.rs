use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TsConvertResolve {
    pub paths: IndexMap<GtPath, GtPath>,
    pub globs: IndexMap<GtPath, String>,
    pub identifiers: IndexMap<GtIdentifier, GtIdentifier>,
    pub boolean_record_key_branded: IndexMap<GtReferenceId, bool>,
}

impl TsConvertResolve {
    pub fn new() -> Self {
        Self {
            paths: IndexMap::new(),
            globs: IndexMap::new(),
            identifiers: IndexMap::new(),
            boolean_record_key_branded: IndexMap::new(),
        }
    }
}

impl Default for TsConvertResolve {
    fn default() -> Self {
        Self::new()
    }
}
