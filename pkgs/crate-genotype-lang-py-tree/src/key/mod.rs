use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyKey(pub Arc<str>);

impl From<&str> for PyKey {
    fn from(str: &str) -> Self {
        PyKey(str.into())
    }
}

impl From<String> for PyKey {
    fn from(str: String) -> Self {
        PyKey(str.into())
    }
}

impl From<Arc<str>> for PyKey {
    fn from(str: Arc<str>) -> Self {
        PyKey(str)
    }
}
