use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSIdentifier(pub String);

impl GtlDependencyRef for TSIdentifier {}

impl From<&str> for TSIdentifier {
    fn from(str: &str) -> Self {
        TSIdentifier(str.into())
    }
}
