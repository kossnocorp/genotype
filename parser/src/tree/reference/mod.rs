use serde::Serialize;

use crate::GTSpan;

use super::{identifier::GTIdentifier, GTReferenceDefinitionId, GTReferenceId};

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GTReference {
    pub span: GTSpan,
    pub id: GTReferenceId,
    pub definition_id: GTReferenceDefinitionId,
    pub identifier: GTIdentifier,
}
