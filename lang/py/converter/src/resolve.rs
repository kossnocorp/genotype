use std::collections::HashMap;

use genotype_parser::tree::{GTIdentifier, GTPath};

#[derive(Debug, PartialEq, Clone)]
pub struct PYConvertResolve {
    pub paths: HashMap<GTPath, GTPath>,
    pub globs: HashMap<GTPath, String>,
    pub identifiers: HashMap<GTIdentifier, GTIdentifier>,
}

impl Default for PYConvertResolve {
    fn default() -> Self {
        Self {
            paths: HashMap::new(),
            globs: HashMap::new(),
            identifiers: HashMap::new(),
        }
    }
}
