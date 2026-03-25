use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsField {
    #[visit]
    pub doc: Option<RsDoc>,
    #[visit]
    pub attributes: Vec<RsAttribute>,
    #[visit]
    pub name: RsFieldName,
    #[visit]
    pub descriptor: RsDescriptor,
}
