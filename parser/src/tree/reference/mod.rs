use super::identifier::GTIdentifier;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTReference(pub GTIdentifier);

impl From<GTIdentifier> for GTReference {
    fn from(identifier: GTIdentifier) -> Self {
        GTReference(identifier)
    }
}
