use crate::GTSpan;

use super::{identifier::GTIdentifier, GTReferenceAliasId};

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTReference(pub GTSpan, pub GTReferenceAliasId, pub GTIdentifier);

impl From<GTIdentifier> for GTReference {
    fn from(identifier: GTIdentifier) -> Self {
        GTReference(
            identifier.0.clone(),
            GTReferenceAliasId::Unresolved,
            identifier,
        )
    }
}
