use crate::prelude::internal::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct TsConvertResolve {
    pub paths: HashMap<GtPath, GtPath>,
    pub globs: HashMap<GtPath, String>,
    pub identifiers: HashMap<GtIdentifier, GtIdentifier>,
}

impl TsConvertResolve {
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
            globs: HashMap::new(),
            identifiers: HashMap::new(),
        }
    }
}

impl Default for TsConvertResolve {
    fn default() -> Self {
        Self::new()
    }
}
