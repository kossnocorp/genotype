use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsDoc(pub String);

impl From<&str> for TsDoc {
    fn from(str: &str) -> Self {
        TsDoc(str.into())
    }
}
