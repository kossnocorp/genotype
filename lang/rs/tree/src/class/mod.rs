use crate::{identifier::RSIdentifier, property::RSProperty, RSDoc, RSExtension};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSClass {
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    pub extensions: Vec<RSExtension>,
    pub properties: Vec<RSProperty>,
    pub references: Vec<RSIdentifier>,
}
