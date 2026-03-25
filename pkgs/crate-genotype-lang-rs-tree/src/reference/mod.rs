use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsReference {
    pub id: GtReferenceId,
    #[visit]
    pub identifier: RsIdentifier,
    pub definition_id: GtDefinitionId,
}

impl RsReference {
    pub fn new(id: GtReferenceId, identifier: RsIdentifier, definition_id: GtDefinitionId) -> Self {
        RsReference {
            id,
            identifier,
            definition_id,
        }
    }
}
