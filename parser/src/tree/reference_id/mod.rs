use crate::GTSpan;

use super::GTModuleId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTReferenceId(pub GTModuleId, pub GTSpan);
