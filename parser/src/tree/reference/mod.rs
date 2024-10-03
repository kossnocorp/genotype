use super::identifier::GTIdentifier;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTReference(GTIdentifier);

impl From<&str> for GTReference {
    fn from(str: &str) -> Self {
        GTReference(str.into())
    }
}
