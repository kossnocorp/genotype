use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTImport {
    pub span: GTSpan,
    #[visit]
    pub path: GTPath,
    #[visit]
    pub reference: GTImportReference,
}
