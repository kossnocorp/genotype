use crate::prelude::internal::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub struct PyConvertResolve {
    pub paths: HashMap<GtPath, GtPath>,
    pub globs: HashMap<GtPath, String>,
    pub identifiers: HashMap<GtIdentifier, GtIdentifier>,
    pub imported: HashSet<GtIdentifier>,
}

impl Default for PyConvertResolve {
    fn default() -> Self {
        Self {
            paths: HashMap::new(),
            globs: HashMap::new(),
            identifiers: HashMap::new(),
            imported: HashSet::new(),
        }
    }
}
