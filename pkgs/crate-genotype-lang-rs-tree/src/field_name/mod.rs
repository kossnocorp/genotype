use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsFieldName(pub Arc<str>);

impl From<&str> for RsFieldName {
    fn from(str: &str) -> Self {
        RsFieldName(str.into())
    }
}

impl From<String> for RsFieldName {
    fn from(str: String) -> Self {
        RsFieldName(str.into())
    }
}

impl From<Arc<str>> for RsFieldName {
    fn from(str: Arc<str>) -> Self {
        RsFieldName(str)
    }
}
