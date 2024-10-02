use crate::identifier::TSIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportName {
    Name(TSIdentifier),
    Alias(TSIdentifier, TSIdentifier),
}
