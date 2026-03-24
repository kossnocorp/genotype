use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTExtension {
    pub span: GTSpan,
    #[visit]
    pub reference: GTReference,
}
