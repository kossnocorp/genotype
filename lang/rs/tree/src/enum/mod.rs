use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSEnum {
    pub id: GTDefinitionId,
    #[visit]
    pub doc: Option<RSDoc>,
    #[visit]
    pub attributes: Vec<RSAttribute>,
    #[visit]
    pub name: RSIdentifier,
    #[visit]
    pub variants: Vec<RSEnumVariant>,
}
