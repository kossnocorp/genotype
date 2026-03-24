use crate::prelude::internal::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
pub struct PYConvertResolve {
    pub paths: HashMap<GTPath, GTPath>,
    pub globs: HashMap<GTPath, String>,
    pub identifiers: HashMap<GTIdentifier, GTIdentifier>,
    pub imported: HashSet<GTIdentifier>,
}

impl Default for PYConvertResolve {
    fn default() -> Self {
        Self {
            paths: HashMap::new(),
            globs: HashMap::new(),
            identifiers: HashMap::new(),
            imported: HashSet::new(),
        }
    }
}
