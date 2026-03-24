use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSKey(pub String);

impl From<&str> for TSKey {
    fn from(str: &str) -> Self {
        TSKey(str.into())
    }
}
