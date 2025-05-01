use super::GTModuleId;

/// Project definition id. It allows to reference and identify entities in
/// the Genotype tree and target trees.
#[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTDefinitionId(
    /// Module id that contains the definition.
    pub GTModuleId,
    /// Source definition name.
    pub String,
);
