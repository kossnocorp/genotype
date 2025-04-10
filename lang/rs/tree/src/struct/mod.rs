use crate::*;
use genotype_parser::GTDefinitionId;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSStruct {
    pub id: GTDefinitionId,
    pub doc: Option<RSDoc>,
    pub attributes: Vec<RSAttribute>,
    pub name: RSIdentifier,
    pub fields: RSStructFields,
}
