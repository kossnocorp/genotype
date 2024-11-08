use std::collections::{HashMap, HashSet};

use genotype_parser::tree::{GTIdentifier, GTPath};

#[derive(Debug, PartialEq, Clone)]
pub struct RSConvertResolve {
    pub paths: HashMap<GTPath, GTPath>,
    pub globs: HashMap<GTPath, String>,
    pub identifiers: HashMap<GTIdentifier, GTIdentifier>,
    pub imported: HashSet<GTIdentifier>,
}

impl Default for RSConvertResolve {
    fn default() -> Self {
        Self {
            paths: HashMap::new(),
            globs: HashMap::new(),
            identifiers: HashMap::new(),
            imported: HashSet::new(),
        }
    }
}
