use std::collections::HashMap;

use genotype_parser::tree::{GTIdentifier, GTPath};

#[derive(Debug, PartialEq, Clone)]
pub struct TSConvertResolve {
    pub modules: HashMap<GTPath, GTPath>,
    pub references: HashMap<GTIdentifier, GTIdentifier>,
}

impl TSConvertResolve {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            references: HashMap::new(),
        }
    }
}
