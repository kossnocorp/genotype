use crate::identifier::RSIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSImportName {
    Name(RSIdentifier),
    Alias(RSIdentifier, RSIdentifier),
}

impl From<&str> for RSImportName {
    fn from(str: &str) -> Self {
        RSImportName::Name(str.into())
    }
}

impl From<RSIdentifier> for RSImportName {
    fn from(identifier: RSIdentifier) -> Self {
        RSImportName::Name(identifier)
    }
}
