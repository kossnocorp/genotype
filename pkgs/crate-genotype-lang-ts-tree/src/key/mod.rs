use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsKey(pub Arc<str>);

impl From<&str> for TsKey {
    fn from(str: &str) -> Self {
        TsKey(str.into())
    }
}

impl From<String> for TsKey {
    fn from(str: String) -> Self {
        TsKey(str.into())
    }
}

impl From<Arc<str>> for TsKey {
    fn from(str: Arc<str>) -> Self {
        TsKey(str)
    }
}
