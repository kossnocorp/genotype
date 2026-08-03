use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtObject {
    pub span: GtSpan,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub attributes: Vec<GtAttribute>,
    #[visit]
    pub name: GtObjectName,
    #[visit]
    pub extensions: Vec<GtExtension>,
    #[visit]
    pub properties: Vec<GtProperty>,
}
