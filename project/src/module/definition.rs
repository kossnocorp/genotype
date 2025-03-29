use std::collections::HashSet;

use genotype_parser::*;

/// Project module definition resolve data.
#[derive(Debug, PartialEq, Clone)]
pub struct GTPModuleDefinitionResolve {
    /// Definition references.
    pub references: HashSet<GTReferenceId>,
    /// Definition dependencies.
    pub deps: HashSet<GTIdentifier>,
}
