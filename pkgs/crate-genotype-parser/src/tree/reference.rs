use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtReference {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    pub id: GtReferenceId,
    #[visit]
    pub identifier: GtIdentifier,
    #[visit]
    pub arguments: Vec<GtGenericArgument>,
}
