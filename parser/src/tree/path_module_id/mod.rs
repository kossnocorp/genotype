use std::hash::Hash;

use serde::Serialize;

use super::GTModuleId;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub enum GTPathModuleId {
    Unresolved,
    Resolved(GTModuleId),
}
