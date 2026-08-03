use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtDoc(pub GtSpan, pub String);

impl GtDoc {
    pub fn new(span: GtSpan, name: String) -> Self {
        Self(span, name)
    }
}
