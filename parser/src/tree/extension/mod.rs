use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTExtension {
    pub span: GTSpan,
    pub reference: GTReference,
}
