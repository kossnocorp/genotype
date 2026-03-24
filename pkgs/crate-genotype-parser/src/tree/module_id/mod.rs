use crate::prelude::internal::*;

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GTModuleId(
    /// Module identifier string.
    pub Arc<str>,
);

impl GTModuleId {
    pub fn definition_id(&self, name: &GTIdentifier) -> GTDefinitionId {
        GTDefinitionId(self.clone(), name.1.clone())
    }
}

impl From<&str> for GTModuleId {
    fn from(s: &str) -> Self {
        GTModuleId(s.into())
    }
}

impl From<String> for GTModuleId {
    fn from(s: String) -> Self {
        GTModuleId(s.into())
    }
}

impl From<Arc<str>> for GTModuleId {
    fn from(s: Arc<str>) -> Self {
        GTModuleId(s)
    }
}
