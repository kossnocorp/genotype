use super::{GTDefinitionId, GTIdentifier};

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTModuleId(
    /// Module identifier string.
    pub String,
);

impl GTModuleId {
    pub fn definition_id(&self, name: &GTIdentifier) -> GTDefinitionId {
        GTDefinitionId(self.clone(), name.as_string())
    }
}

impl From<&str> for GTModuleId {
    fn from(s: &str) -> Self {
        GTModuleId(s.to_string())
    }
}

impl From<String> for GTModuleId {
    fn from(s: String) -> Self {
        GTModuleId(s)
    }
}
