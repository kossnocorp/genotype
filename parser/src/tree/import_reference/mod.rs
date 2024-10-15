use crate::GTSpan;

use super::{identifier::GTIdentifier, import_name::GTImportName};

#[derive(Debug, PartialEq, Clone)]
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
