use std::hash::Hash;

use serde::Serialize;

use super::GTDefinitionId;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub enum GTReferenceDefinitionId {
    Unresolved,
    Resolved(GTDefinitionId),
}
