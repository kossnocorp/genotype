use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtGenericParameter {
    pub span: GtSpan,
    #[visit]
    pub identifier: GtIdentifier,
}
