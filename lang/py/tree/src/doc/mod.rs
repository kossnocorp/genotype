use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYDoc(pub String);

impl From<&str> for PYDoc {
    fn from(str: &str) -> Self {
        PYDoc(str.into())
    }
}
