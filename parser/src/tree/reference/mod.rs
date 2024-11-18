use crate::GTSpan;

use super::{identifier::GTIdentifier, GTReferenceDefinitionId};

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTReference(pub GTSpan, pub GTReferenceDefinitionId, pub GTIdentifier);
