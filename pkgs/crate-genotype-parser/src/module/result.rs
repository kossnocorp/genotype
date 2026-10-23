use crate::prelude::internal::*;

/// Module parse result. It contains the module tree and resolve data.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtModuleParse {
    /// Module tree.
    pub module: GtModule,
    /// Module resolve. It contains module meta information used to build
    /// the dependency graph.
    pub resolve: GtModuleResolve,
}
