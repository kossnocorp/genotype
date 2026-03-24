use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSDoc(pub String);

impl From<&str> for TSDoc {
    fn from(str: &str) -> Self {
        TSDoc(str.into())
    }
}
