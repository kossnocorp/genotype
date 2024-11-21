use genotype_parser::GTDefinitionId;

use crate::{identifier::RSIdentifier, RSAttribute, RSDescriptor, RSDoc};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSNewtype {
    pub id: GTDefinitionId,
    pub doc: Option<RSDoc>,
    pub attributes: Vec<RSAttribute>,
    pub name: RSIdentifier,
    pub descriptors: Vec<RSDescriptor>,
}
