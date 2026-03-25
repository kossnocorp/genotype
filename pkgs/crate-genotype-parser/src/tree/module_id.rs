use crate::prelude::internal::*;

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GtModuleId(
    /// Module identifier string.
    pub Arc<str>,
);

impl GtModuleId {
    pub fn definition_id(&self, name: &GtIdentifier) -> GtDefinitionId {
        GtDefinitionId(self.clone(), name.1.clone())
    }
}

impl From<&str> for GtModuleId {
    fn from(s: &str) -> Self {
        GtModuleId(s.into())
    }
}

impl From<String> for GtModuleId {
    fn from(s: String) -> Self {
        GtModuleId(s.into())
    }
}

impl From<Arc<str>> for GtModuleId {
    fn from(s: Arc<str>) -> Self {
        GtModuleId(s)
    }
}
