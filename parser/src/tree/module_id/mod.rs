use super::{GTAliasId, GTIdentifier};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTModuleId(pub String);

impl GTModuleId {
    pub fn alias_id(&self, name: &GTIdentifier) -> GTAliasId {
        GTAliasId(self.clone(), name.as_string())
    }
}

impl From<&str> for GTModuleId {
    fn from(s: &str) -> Self {
        GTModuleId(s.to_string())
    }
}
