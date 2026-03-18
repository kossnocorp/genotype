use serde::Serialize;

use crate::GTSpan;

use super::GTReference;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTExtension {
    pub span: GTSpan,
    pub reference: GTReference,
}
