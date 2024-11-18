use genotype_parser::GTAliasId;

use crate::{descriptor::RSDescriptor, identifier::RSIdentifier, RSDoc};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSAlias {
    pub id: GTAliasId,
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub descriptor: RSDescriptor,
}
