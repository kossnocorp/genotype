use crate::{descriptor::RSDescriptor, field_name::RSFieldName, RSAttribute, RSDoc};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSField {
    pub doc: Option<RSDoc>,
    pub attributes: Vec<RSAttribute>,
    pub name: RSFieldName,
    pub descriptor: RSDescriptor,
}
