use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTObject {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub attributes: Vec<GTAttribute>,
    pub name: GTObjectName,
    pub extensions: Vec<GTExtension>,
    pub properties: Vec<GTProperty>,
}
