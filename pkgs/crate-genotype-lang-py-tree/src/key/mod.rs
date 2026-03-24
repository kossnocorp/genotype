use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYKey(pub Arc<str>);

impl From<&str> for PYKey {
    fn from(str: &str) -> Self {
        PYKey(str.into())
    }
}

impl From<String> for PYKey {
    fn from(str: String) -> Self {
        PYKey(str.into())
    }
}

impl From<Arc<str>> for PYKey {
    fn from(str: Arc<str>) -> Self {
        PYKey(str)
    }
}
