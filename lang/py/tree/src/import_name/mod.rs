use crate::identifier::PYIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYImportName {
    Name(PYIdentifier),
    Alias(PYIdentifier, PYIdentifier),
}
