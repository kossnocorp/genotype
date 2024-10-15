use crate::GTSpan;

use super::identifier::GTIdentifier;

#[derive(Debug, PartialEq, Clone)]
pub enum GTImportName {
    Name(GTSpan, GTIdentifier),
    Alias(GTSpan, GTIdentifier, GTIdentifier),
}
