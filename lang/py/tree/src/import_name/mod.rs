use crate::identifier::PYIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYImportName {
    Name(PYIdentifier),
    Alias(PYIdentifier, PYIdentifier),
}

impl From<&str> for PYImportName {
    fn from(str: &str) -> Self {
        PYImportName::Name(str.into())
    }
}

impl From<PYIdentifier> for PYImportName {
    fn from(identifier: PYIdentifier) -> Self {
        PYImportName::Name(identifier)
    }
}
