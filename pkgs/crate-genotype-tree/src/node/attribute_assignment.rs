use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeAssignment {
    pub span: GtSpan,
    pub value: GtAttributeValue,
}

impl GtAttributeAssignment {
    pub fn new(span: GtSpan, value: GtAttributeValue) -> Self {
        Self { span, value }
    }
}
