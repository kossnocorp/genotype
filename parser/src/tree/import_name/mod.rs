use super::identifier::GTIdentifier;

#[derive(Debug, PartialEq, Clone)]
pub enum GTImportName {
    Name(GTIdentifier),
    Alias(GTIdentifier, GTIdentifier),
}
