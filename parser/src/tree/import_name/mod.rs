use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GTImportName {
    Name(GTSpan, GTIdentifier),
    Alias(GTSpan, GTIdentifier, GTIdentifier),
}
