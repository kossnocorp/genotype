use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct RSAlias {
    pub id: GTDefinitionId,
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub descriptor: RSDescriptor,
}
