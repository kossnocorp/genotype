use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtExtension {
    pub span: GtSpan,
    #[visit]
    pub reference: GtReference,
}
