use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct RSPath(pub GTModuleId, pub Arc<str>);

impl RSPath {
    pub fn join(str: &str, other: &str) -> String {
        format!("{}::{}", str, other)
    }
}
