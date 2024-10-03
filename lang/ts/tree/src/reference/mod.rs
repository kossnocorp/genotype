use crate::identifier::TSIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSReference(pub TSIdentifier);

impl From<&str> for TSReference {
    fn from(str: &str) -> Self {
        TSReference(str.into())
    }
}
