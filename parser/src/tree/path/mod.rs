use std::hash::Hash;

use crate::GTSpan;

use super::GTPathModuleId;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTPath(pub GTSpan, pub GTPathModuleId, String);

impl GTPath {
    pub fn as_str(&self) -> &str {
        &self.2
    }
}
