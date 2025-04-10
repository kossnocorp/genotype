use crate::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSEnumVariant {
    pub doc: Option<RSDoc>,
    pub attributes: Vec<RSAttribute>,
    pub name: RSIdentifier,
    pub descriptor: RSEnumVariantDescriptor,
}
