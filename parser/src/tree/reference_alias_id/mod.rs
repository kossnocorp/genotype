use std::hash::Hash;

use super::GTAliasId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum GTReferenceAliasId {
    Unresolved,
    Resolved(GTAliasId),
}
