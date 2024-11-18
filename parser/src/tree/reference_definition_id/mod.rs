use std::hash::Hash;

use super::GTDefinitionId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum GTReferenceDefinitionId {
    Unresolved,
    Resolved(GTDefinitionId),
}
