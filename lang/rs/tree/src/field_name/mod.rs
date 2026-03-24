use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSFieldName(pub String);

impl From<&str> for RSFieldName {
    fn from(str: &str) -> Self {
        RSFieldName(str.into())
    }
}
