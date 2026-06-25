use crate::prelude::internal::*;

use std::sync::Arc;

#[derive(Default, Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GtModuleId(
    /// Module identifier string.
    pub Arc<str>,
);

impl GtModuleId {
    pub fn as_str_without_ext(&self) -> String {
        self.0.trim_end_matches(".type").to_owned()
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
