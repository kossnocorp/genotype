use std::hash::Hash;

use crate::GTSpan;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTPath(pub GTSpan, String);

impl GTPath {
    pub fn as_str(&self) -> &str {
        &self.1
    }
}
