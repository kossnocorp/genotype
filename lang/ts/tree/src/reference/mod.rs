use crate::{identifier::TSIdentifier, path::TSPath};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSReference {
    Unresolved(TSIdentifier),
    Local(TSIdentifier),
    External(TSIdentifier, TSPath),
}
