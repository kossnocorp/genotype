use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtKey(pub GtSpan, pub Arc<str>);

impl GtKey {
    pub fn new(span: GtSpan, value: Arc<str>) -> Self {
        Self(span, value)
    }
}
