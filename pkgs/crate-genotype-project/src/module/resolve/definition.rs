use crate::prelude::internal::*;

/// Definition resolve data in the context of the current module. It contains lists all the module
/// definition references and the definition dependencies.
#[derive(Default, Debug, PartialEq, Clone, Serialize)]
pub struct GtpModuleResolveDefinition {
    /// All definition references in the module.
    pub references: IndexSet<GtReferenceId>,
    /// Identifiers the definition depends on.
    pub deps: IndexSet<GtIdentifier>,
}
