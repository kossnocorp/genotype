use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYKey(pub String);

impl From<&str> for PYKey {
    fn from(str: &str) -> Self {
        PYKey(str.into())
    }
}
