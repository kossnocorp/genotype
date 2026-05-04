use crate::prelude::internal::*;

/// Module resolve data. It contains module meta information used to build the dependency graph
/// connecting modules in a project.
#[derive(Default, Debug, PartialEq, Clone, Serialize)]
pub struct GtModuleResolve {
    pub deps: IndexSet<GtPath>,
    pub exports: Vec<GtIdentifier>,
    pub references: IndexSet<GtIdentifier>,
    /// Identifiers marked as generic parameters in the current module. It allows to resolve generic
    /// parameter references.
    pub generic_parameters: IndexSet<GtIdentifier>,
}

impl GtModuleResolve {
    pub fn new() -> Self {
        Self::default()
    }
}
