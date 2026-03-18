use crate::prelude::internal::*;

use std::collections::HashSet;

use genotype_parser::*;

/// Definition resolve data in the context of the current module. It contains lists all the module
/// definition references and the definition dependencies.
#[derive(Default, Debug, PartialEq, Clone, Serialize)]
pub struct GtProjectModuleDefinitionResolve {
    /// All definition references in the module.
    pub references: HashSet<GTReferenceId>,
    /// Identfiers the definition depends on.
    pub deps: HashSet<GTIdentifier>,
}
