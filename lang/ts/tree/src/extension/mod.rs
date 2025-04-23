use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSExtension {
    pub reference: TSReference,
}

impl From<&str> for TSExtension {
    fn from(str: &str) -> Self {
        TSExtension {
            reference: str.into(),
        }
    }
}
