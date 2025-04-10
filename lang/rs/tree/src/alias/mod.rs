use crate::*;
use genotype_parser::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSAlias {
    pub id: GTDefinitionId,
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub descriptor: RSDescriptor,
}
