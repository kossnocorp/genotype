use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTAny {
    pub span: GTSpan,
    pub doc: Option<GTDoc>,
    pub attributes: Vec<GTAttribute>,
}
