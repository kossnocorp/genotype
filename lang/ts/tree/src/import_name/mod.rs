use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportName {
    Name(TSIdentifier),
    Alias(TSIdentifier, TSIdentifier),
}

impl From<&str> for TSImportName {
    fn from(str: &str) -> Self {
        TSImportName::Name(str.into())
    }
}
