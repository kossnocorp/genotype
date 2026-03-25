use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub enum GtReferenceDefinitionId {
    Unresolved,
    Resolved(GtDefinitionId),
}
