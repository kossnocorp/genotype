use crate::{descriptor::RSDescriptor, key::RSKey, RSDoc};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSProperty {
    pub doc: Option<RSDoc>,
    pub name: RSKey,
    pub descriptor: RSDescriptor,
    pub required: bool,
}
