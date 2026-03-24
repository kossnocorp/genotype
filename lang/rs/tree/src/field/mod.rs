use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSField {
    #[visit]
    pub doc: Option<RSDoc>,
    #[visit]
    pub attributes: Vec<RSAttribute>,
    #[visit]
    pub name: RSFieldName,
    #[visit]
    pub descriptor: RSDescriptor,
}
