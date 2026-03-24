use crate::prelude::internal::*;

mod parse;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTProperty {
    pub span: GTSpan,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub attributes: Vec<GTAttribute>,
    #[visit]
    pub name: GTKey,
    #[visit]
    pub descriptor: GTDescriptor,
    pub required: bool,
}
