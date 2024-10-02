use super::{identifier::GTIdentifier, path::GTPath};

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum GTReference {
    Unresolved(GTIdentifier),
    Local(GTIdentifier),
    External(GTIdentifier, GTPath),
}
