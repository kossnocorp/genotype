use crate::{descriptor::RSDescriptor, key::RSKey, RSAttribute, RSDoc};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSProperty {
    pub doc: Option<RSDoc>,
    pub attributes: Vec<RSAttribute>,
    pub name: RSKey,
    pub descriptor: RSDescriptor,
}
