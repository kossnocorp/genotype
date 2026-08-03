use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtInlineImport {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub name: GtIdentifier,
    #[visit]
    pub arguments: Vec<GtGenericArgument>,
    #[visit]
    pub path: GtPath,
}
