use crate::prelude::internal::*;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GTReference {
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    pub id: GTReferenceId,
    pub definition_id: GTReferenceDefinitionId,
    #[visit]
    pub identifier: GTIdentifier,
}
