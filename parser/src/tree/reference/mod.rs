use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GTReference {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub attributes: Vec<GTAttribute>,
    pub id: GTReferenceId,
    pub definition_id: GTReferenceDefinitionId,
    pub identifier: GTIdentifier,
}
