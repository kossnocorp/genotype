use crate::identifier::RSIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSUseName {
    Name(RSIdentifier),
    Alias(RSIdentifier, RSIdentifier),
}

impl From<&str> for RSUseName {
    fn from(str: &str) -> Self {
        RSUseName::Name(str.into())
    }
}

impl From<RSIdentifier> for RSUseName {
    fn from(identifier: RSIdentifier) -> Self {
        RSUseName::Name(identifier)
    }
}
