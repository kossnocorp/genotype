use std::collections::HashSet;

use super::{GTIdentifier, GTPath};

#[derive(Debug, PartialEq, Clone)]
pub struct GTResolve {
    pub deps: HashSet<GTPath>,
    pub exports: Vec<GTIdentifier>,
    pub references: HashSet<GTIdentifier>,
}

impl GTResolve {
    pub fn new() -> Self {
        GTResolve {
            deps: HashSet::new(),
            exports: Vec::new(),
            references: HashSet::new(),
        }
    }
}
