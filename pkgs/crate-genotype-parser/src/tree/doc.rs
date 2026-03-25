use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtDoc(pub GtSpan, pub String);

impl GtDoc {
    pub fn new(span: GtSpan, name: String) -> Self {
        Self(span, name)
    }
}

impl GtDoc {
    pub fn concat(&self, pair: Pair<'_, Rule>) -> Self {
        let added_span = pair.as_span();
        let span = (self.0.0, added_span.end()).into();
        GtDoc(span, format!("{}\n{}", self.1, pair.as_str()))
    }
}

impl From<Pair<'_, Rule>> for GtDoc {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let span = pair.as_span().into();
        GtDoc(span, pair.as_str().into())
    }
}
