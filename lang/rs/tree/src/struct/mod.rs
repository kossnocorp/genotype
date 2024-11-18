use genotype_parser::GTAliasId;

use crate::{identifier::RSIdentifier, RSAttribute, RSDoc, RSStructFields};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSStruct {
    pub id: GTAliasId,
    pub doc: Option<RSDoc>,
    pub attributes: Vec<RSAttribute>,
    pub name: RSIdentifier,
    pub fields: RSStructFields,
}
