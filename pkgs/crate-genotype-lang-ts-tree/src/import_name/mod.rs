use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TSImportName {
    Name(#[visit] TSIdentifier),
    Alias(#[visit] TSIdentifier, #[visit] TSIdentifier),
}

impl From<&str> for TSImportName {
    fn from(str: &str) -> Self {
        TSImportName::Name(str.into())
    }
}
