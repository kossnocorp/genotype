use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct PyPath(pub Arc<str>);

impl From<&str> for PyPath {
    fn from(str: &str) -> Self {
        PyPath(str.into())
    }
}

impl From<String> for PyPath {
    fn from(str: String) -> Self {
        PyPath(str.into())
    }
}

impl From<Arc<str>> for PyPath {
    fn from(str: Arc<str>) -> Self {
        PyPath(str)
    }
}
