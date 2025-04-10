use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSReference(pub TSIdentifier);

impl From<&str> for TSReference {
    fn from(str: &str) -> Self {
        TSReference(str.into())
    }
}

impl From<TSIdentifier> for TSReference {
    fn from(identifier: TSIdentifier) -> Self {
        TSReference(identifier)
    }
}
