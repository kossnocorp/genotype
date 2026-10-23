use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeKey {
    pub span: GtSpan,
    pub value: Arc<str>,
}

impl GtAttributeKey {
    pub fn new(span: GtSpan, name: Arc<str>) -> Self {
        Self { span, value: name }
    }
}
