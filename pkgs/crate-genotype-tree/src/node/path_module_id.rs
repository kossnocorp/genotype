use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GtPathModuleId {
    /// Path position in the source code.
    pub span: GtSpan,
    /// Module id that contains the path.
    pub module_id: GtModuleId,
}

impl GtPathModuleId {
    pub fn new(span: GtSpan, module_id: GtModuleId) -> Self {
        Self { span, module_id }
    }
}
