use super::GTModuleId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTAliasId(pub GTModuleId, pub String);
