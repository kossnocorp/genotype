use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsExtension {
    #[visit]
    pub reference: TsReference,
}

impl From<&str> for TsExtension {
    fn from(str: &str) -> Self {
        TsExtension {
            reference: str.into(),
        }
    }
}
