use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct PYPath(pub String);

impl From<&str> for PYPath {
    fn from(str: &str) -> Self {
        PYPath(str.into())
    }
}
