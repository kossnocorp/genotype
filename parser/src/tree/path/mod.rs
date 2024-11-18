use std::hash::Hash;

use crate::GTSpan;

use super::GTPathModuleId;

mod parse;

#[derive(Debug, Eq, Hash, Clone)]
pub struct GTPath(pub GTSpan, pub GTPathModuleId, String);

impl GTPath {
    pub fn as_str(&self) -> &str {
        &self.2
    }
}

impl PartialEq for GTPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.2 == other.2
    }
}
