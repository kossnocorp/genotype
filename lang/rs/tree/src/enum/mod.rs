use genotype_parser::GTDefinitionId;

use crate::{RSAttribute, RSDoc, RSEnumVariant, RSIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSEnum {
    pub id: GTDefinitionId,
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub attributes: Vec<RSAttribute>,
    pub variants: Vec<RSEnumVariant>,
}
