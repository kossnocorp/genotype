use miette::SourceSpan;
use pest::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct GTSpan(pub usize, pub usize);

impl GTSpan {
    pub fn offset(&self) -> usize {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1 - self.0
    }
}

impl From<(usize, usize)> for GTSpan {
    fn from((start, end): (usize, usize)) -> Self {
        GTSpan(start, end)
    }
}

impl From<Span<'_>> for GTSpan {
    fn from(span: Span<'_>) -> Self {
        GTSpan(span.start(), span.end())
    }
}

impl Into<SourceSpan> for GTSpan {
    fn into(self) -> SourceSpan {
        (self.offset(), self.len()).into()
    }
}

impl Default for GTSpan {
    fn default() -> Self {
        GTSpan(0, 0)
    }
}
