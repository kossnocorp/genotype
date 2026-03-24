use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSKey(pub Arc<str>);

impl From<&str> for TSKey {
    fn from(str: &str) -> Self {
        TSKey(str.into())
    }
}

impl From<String> for TSKey {
    fn from(str: String) -> Self {
        TSKey(str.into())
    }
}

impl From<Arc<str>> for TSKey {
    fn from(str: Arc<str>) -> Self {
        TSKey(str)
    }
}
