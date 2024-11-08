use crate::{descriptor::RSDescriptor, key::RSKey, RSDoc};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSProperty {
    pub doc: Option<RSDoc>,
    // [TODO] Attributes
    pub name: RSKey,
    pub descriptor: RSDescriptor,
}
