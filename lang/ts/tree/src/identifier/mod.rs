use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSIdentifier(pub String);

impl GtlDependencyRef for TSIdentifier {}

impl From<&str> for TSIdentifier {
    fn from(str: &str) -> Self {
        TSIdentifier(str.into())
    }
}

impl From<String> for TSIdentifier {
    fn from(str: String) -> Self {
        TSIdentifier(str)
    }
}
