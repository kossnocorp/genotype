use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTObject {
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    #[visit]
    pub name: GTObjectName,
    #[visit]
    pub extensions: Vec<GTExtension>,
    #[visit]
    pub properties: Vec<GTProperty>,
}
