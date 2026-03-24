use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum GTImportReference {
    Glob(GTSpan),
    Names(GTSpan, #[visit] Vec<GTImportName>),
    Name(GTSpan, #[visit] GTIdentifier),
}

impl From<GTIdentifier> for GTImportReference {
    fn from(identifier: GTIdentifier) -> Self {
        GTImportReference::Name(identifier.0.clone(), identifier)
    }
}
