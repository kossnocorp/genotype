use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize, Default)]
pub struct GtSpan(pub usize, pub usize);

impl GtSpan {
    pub fn offset(&self) -> usize {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1 - self.0
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<(usize, usize)> for GtSpan {
    fn from((start, end): (usize, usize)) -> Self {
        GtSpan(start, end)
    }
}

impl From<Span<'_>> for GtSpan {
    fn from(span: Span<'_>) -> Self {
        GtSpan(span.start(), span.end())
    }
}

impl From<GtSpan> for SourceSpan {
    fn from(val: GtSpan) -> Self {
        (val.offset(), val.len()).into()
    }
}
