use std::hash::Hash;

use super::GTModuleId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum GTPathModuleId {
    Unresolved,
    Resolved(GTModuleId),
}
