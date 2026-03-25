use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct RsPath(pub GtModuleId, pub Arc<str>);

impl RsPath {
    pub fn join(str: &str, other: &str) -> String {
        format!("{}::{}", str, other)
    }
}
