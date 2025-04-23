use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportName {
    Name(TSIdentifier),
    Alias(TSIdentifier, TSIdentifier),
}
