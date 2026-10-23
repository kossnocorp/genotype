use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum GtImportName {
    Name(GtSpan, #[visit] GtIdentifier),
    Alias(GtSpan, #[visit] GtIdentifier, #[visit] GtIdentifier),
}
