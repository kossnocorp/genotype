use crate::GTSpan;

use super::GTReference;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTExtension {
    pub span: GTSpan,
    pub reference: GTReference,
}
