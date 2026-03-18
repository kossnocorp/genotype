use serde::Serialize;

use crate::GTSpan;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GTKey(pub GTSpan, pub String);

impl GTKey {
    pub fn new(span: GTSpan, name: String) -> Self {
        Self(span, name)
    }
}
