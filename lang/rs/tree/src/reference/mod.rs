use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSReference {
    pub id: GTReferenceId,
    pub identifier: RSIdentifier,
    pub definition_id: GTDefinitionId,
}

impl RSReference {
    pub fn new(id: GTReferenceId, identifier: RSIdentifier, definition_id: GTDefinitionId) -> Self {
        RSReference {
            id,
            identifier,
            definition_id,
        }
    }
}
