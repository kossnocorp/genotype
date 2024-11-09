use crate::{descriptor::RSDescriptor, identifier::RSIdentifier, RSDoc};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSAlias {
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub descriptor: RSDescriptor,
}
