use std::collections::HashSet;

use super::{GTIdentifier, GTPath};

/// Module resolve data. It contains module meta information used to build the dependency graph
/// connecting modules in a project.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct GTModuleResolve {
    pub deps: HashSet<GTPath>,
    pub exports: Vec<GTIdentifier>,
    pub references: HashSet<GTIdentifier>,
}

impl GTModuleResolve {
    pub fn new() -> Self {
        Self::default()
    }
}
