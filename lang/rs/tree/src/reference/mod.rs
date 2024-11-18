use genotype_parser::GTDefinitionId;

use crate::identifier::RSIdentifier;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSReference {
    pub identifier: RSIdentifier,
    pub definition_id: GTDefinitionId,
}

impl RSReference {
    pub fn new(identifier: RSIdentifier, definition_id: GTDefinitionId) -> Self {
        RSReference {
            identifier,
            definition_id,
        }
    }
}
