use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSFieldName(pub Arc<str>);

impl From<&str> for RSFieldName {
    fn from(str: &str) -> Self {
        RSFieldName(str.into())
    }
}

impl From<String> for RSFieldName {
    fn from(str: String) -> Self {
        RSFieldName(str.into())
    }
}

impl From<Arc<str>> for RSFieldName {
    fn from(str: Arc<str>) -> Self {
        RSFieldName(str)
    }
}
