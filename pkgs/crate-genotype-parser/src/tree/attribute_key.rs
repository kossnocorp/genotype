use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeKey {
    pub span: GtSpan,
    pub value: Arc<str>,
}
