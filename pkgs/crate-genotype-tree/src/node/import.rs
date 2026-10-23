use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtImport {
    pub span: GtSpan,
    #[visit]
    pub path: GtPath,
    #[visit]
    pub reference: GtImportReference,
}
