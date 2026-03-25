use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct GtSpan(pub usize, pub usize);

impl GtSpan {
    pub fn offset(&self) -> usize {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1 - self.0
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

impl Into<SourceSpan> for GtSpan {
    fn into(self) -> SourceSpan {
        (self.offset(), self.len()).into()
    }
}

impl Default for GtSpan {
    fn default() -> Self {
        GtSpan(0, 0)
    }
}
