use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSAlias {
    pub id: GTDefinitionId,
    #[visit]
    pub doc: Option<RSDoc>,
    #[visit]
    pub name: RSIdentifier,
    #[visit]
    pub descriptor: RSDescriptor,
}
