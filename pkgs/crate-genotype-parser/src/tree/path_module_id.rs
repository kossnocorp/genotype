use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub enum GTPathModuleId {
    Unresolved,
    Resolved(GTModuleId),
}
