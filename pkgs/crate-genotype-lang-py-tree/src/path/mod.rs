use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct PYPath(pub Arc<str>);

impl From<&str> for PYPath {
    fn from(str: &str) -> Self {
        PYPath(str.into())
    }
}

impl From<String> for PYPath {
    fn from(str: String) -> Self {
        PYPath(str.into())
    }
}

impl From<Arc<str>> for PYPath {
    fn from(str: Arc<str>) -> Self {
        PYPath(str)
    }
}
