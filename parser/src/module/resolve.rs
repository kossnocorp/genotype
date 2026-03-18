use indexmap::IndexSet;
use serde::Serialize;

use super::{GTIdentifier, GTPath};

/// Module resolve data. It contains module meta information used to build the dependency graph
/// connecting modules in a project.
#[derive(Default, Debug, PartialEq, Clone, Serialize)]
pub struct GTModuleResolve {
    pub deps: IndexSet<GTPath>,
    pub exports: Vec<GTIdentifier>,
    pub references: IndexSet<GTIdentifier>,
}

impl GTModuleResolve {
    pub fn new() -> Self {
        Self::default()
    }
}
