use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GtModuleSource {
    pub span: GtSpan,
    pub path: GtPath,
}

impl GtModuleSource {
    pub fn new(span: &GtSpan, path: &GtPath) -> Self {
        Self {
            span: *span,
            path: path.clone(),
        }
    }
}
