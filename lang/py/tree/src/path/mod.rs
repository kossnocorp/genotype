mod convert;
pub use convert::*;
use serde::Serialize;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct PYPath(pub String);

impl From<&str> for PYPath {
    fn from(str: &str) -> Self {
        PYPath(str.into())
    }
}
