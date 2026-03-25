use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsAttribute(pub String);

impl From<&str> for RsAttribute {
    fn from(value: &str) -> Self {
        RsAttribute(value.into())
    }
}

impl From<String> for RsAttribute {
    fn from(value: String) -> Self {
        RsAttribute(value)
    }
}
