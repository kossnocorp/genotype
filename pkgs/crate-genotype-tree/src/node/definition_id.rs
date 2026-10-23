use crate::prelude::internal::*;

/// Project definition id. It allows to reference and identify entities in
/// the Genotype tree and target trees.
#[derive(Default, Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GtDefinitionId(
    /// Module id that contains the definition.
    pub GtModuleId,
    /// Source definition name.
    pub Arc<str>,
);
