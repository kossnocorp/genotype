use crate::identifier::RSIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSUseName {
    Name(RSIdentifier),
    Alias(RSIdentifier, RSIdentifier),
}

impl RSUseName {
    pub fn name(&self) -> &RSIdentifier {
        match self {
            RSUseName::Name(name) => name,
            RSUseName::Alias(_, name) => name,
        }
    }

    pub fn original_name(&self) -> &RSIdentifier {
        match self {
            RSUseName::Name(name) => name,
            RSUseName::Alias(name, _) => name,
        }
    }
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
