use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSEnumVariant {
    #[visit]
    pub doc: Option<RSDoc>,
    #[visit]
    pub attributes: Vec<RSAttribute>,
    #[visit]
    pub name: RSIdentifier,
    #[visit]
    pub descriptor: Option<RSEnumVariantDescriptor>,
}
