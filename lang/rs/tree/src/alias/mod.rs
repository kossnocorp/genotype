use genotype_parser::GTDefinitionId;

use crate::{descriptor::RSDescriptor, identifier::RSIdentifier, RSDoc};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSAlias {
    pub id: GTDefinitionId,
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub descriptor: RSDescriptor,
}
