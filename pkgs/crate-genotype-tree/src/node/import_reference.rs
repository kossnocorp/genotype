use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum GtImportReference {
    Glob(GtSpan),
    Names(GtSpan, #[visit] Vec<GtImportName>),
    Name(GtSpan, #[visit] GtIdentifier),
}

impl From<GtIdentifier> for GtImportReference {
    fn from(identifier: GtIdentifier) -> Self {
        GtImportReference::Name(identifier.0, identifier)
    }
}
