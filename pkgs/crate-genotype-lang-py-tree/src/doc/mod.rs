use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyDoc(pub String);

impl From<&str> for PyDoc {
    fn from(str: &str) -> Self {
        PyDoc(str.into())
    }
}
