use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum GTImportName {
    Name(GTSpan, #[visit] GTIdentifier),
    Alias(GTSpan, #[visit] GTIdentifier, #[visit] GTIdentifier),
}
