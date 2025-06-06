use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, PartialOrd, Ord)]
pub struct PYIdentifier(pub String);

impl GtlDependencyRef for PYIdentifier {}

impl From<&str> for PYIdentifier {
    fn from(str: &str) -> Self {
        PYIdentifier(str.into())
    }
}

impl From<String> for PYIdentifier {
    fn from(str: String) -> Self {
        PYIdentifier(str)
    }
}
