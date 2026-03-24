use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTKey(pub GTSpan, pub Arc<str>);

impl GTKey {
    pub fn new(span: GTSpan, name: Arc<str>) -> Self {
        Self(span, name)
    }
}
