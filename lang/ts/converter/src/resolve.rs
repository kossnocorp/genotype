use std::collections::HashMap;

use genotype_parser::tree::{GTIdentifier, GTPath};

#[derive(Debug, PartialEq, Clone)]
pub struct TSConvertResolve {
    pub paths: HashMap<GTPath, GTPath>,
    pub identifiers: HashMap<GTIdentifier, GTIdentifier>,
}

impl TSConvertResolve {
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
            identifiers: HashMap::new(),
        }
    }
}
