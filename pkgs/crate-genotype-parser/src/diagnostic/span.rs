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

impl From<std::ops::Range<usize>> for GtSpan {
    fn from(val: std::ops::Range<usize>) -> Self {
        GtSpan(val.start, val.end)
    }
}

impl From<GtSpan> for SourceSpan {
    fn from(val: GtSpan) -> Self {
        (val.offset(), val.len()).into()
    }
}

impl chumsky::span::Span for GtSpan {
    type Context = ();
    type Offset = usize;

    fn new(_: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        range.into()
    }
    fn context(&self) -> Self::Context {}
    fn start(&self) -> Self::Offset {
        self.0
    }
    fn end(&self) -> Self::Offset {
        self.1
    }
}
