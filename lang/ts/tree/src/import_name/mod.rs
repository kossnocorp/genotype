use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum TSImportName {
    Name(TSIdentifier),
    Alias(TSIdentifier, TSIdentifier),
}

impl From<&str> for TSImportName {
    fn from(str: &str) -> Self {
        TSImportName::Name(str.into())
    }
}
