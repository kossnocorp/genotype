use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTImport {
    pub span: GTSpan,
    pub path: GTPath,
    pub reference: GTImportReference,
}
