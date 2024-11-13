use crate::{RSAttribute, RSDoc, RSEnumVariant, RSIdentifier};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSEnum {
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub attributes: Vec<RSAttribute>,
    pub variants: Vec<RSEnumVariant>,
}
