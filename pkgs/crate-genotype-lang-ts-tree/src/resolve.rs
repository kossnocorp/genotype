use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TsConvertResolve {
    pub paths: IndexMap<GtPath, GtPath>,
    pub globs: IndexMap<GtPath, String>,
    pub identifiers: IndexMap<GtIdentifier, GtIdentifier>,
}

impl TsConvertResolve {
    pub fn new() -> Self {
        Self {
            paths: IndexMap::new(),
            globs: IndexMap::new(),
            identifiers: IndexMap::new(),
        }
    }
}

impl Default for TsConvertResolve {
    fn default() -> Self {
        Self::new()
    }
}
