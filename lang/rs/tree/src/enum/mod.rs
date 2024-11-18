use genotype_parser::GTAliasId;

use crate::{RSAttribute, RSDoc, RSEnumVariant, RSIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSEnum {
    pub id: GTAliasId,
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub attributes: Vec<RSAttribute>,
    pub variants: Vec<RSEnumVariant>,
}
