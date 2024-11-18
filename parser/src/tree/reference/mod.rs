use crate::GTSpan;

use super::{identifier::GTIdentifier, GTReferenceDefinitionId};

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTReference(pub GTSpan, pub GTReferenceDefinitionId, pub GTIdentifier);

impl From<GTIdentifier> for GTReference {
    fn from(identifier: GTIdentifier) -> Self {
        GTReference(
            identifier.0.clone(),
            GTReferenceDefinitionId::Unresolved,
            identifier,
        )
    }
}
