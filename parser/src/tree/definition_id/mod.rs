use super::GTModuleId;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTDefinitionId(pub GTModuleId, pub String);
