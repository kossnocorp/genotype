use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeName {
    pub span: GtSpan,
    pub value: Arc<str>,
}

impl GtAttributeName {
    pub fn new(span: GtSpan, value: Arc<str>) -> Self {
        Self { span, value }
    }
}
