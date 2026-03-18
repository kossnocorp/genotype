use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GTImportReference {
    Glob(GTSpan),
    Names(GTSpan, Vec<GTImportName>),
    Name(GTSpan, GTIdentifier),
}

impl From<GTIdentifier> for GTImportReference {
    fn from(identifier: GTIdentifier) -> Self {
        GTImportReference::Name(identifier.0.clone(), identifier)
    }
}
