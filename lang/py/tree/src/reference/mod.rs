use crate::identifier::PYIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYReference(pub PYIdentifier);

impl From<&str> for PYReference {
    fn from(str: &str) -> Self {
        PYReference(str.into())
    }
}
