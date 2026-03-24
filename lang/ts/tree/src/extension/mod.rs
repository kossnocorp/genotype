use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSExtension {
    #[visit]
    pub reference: TSReference,
}

impl From<&str> for TSExtension {
    fn from(str: &str) -> Self {
        TSExtension {
            reference: str.into(),
        }
    }
}
