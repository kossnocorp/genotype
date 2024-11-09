use crate::{identifier::RSIdentifier, property::RSProperty, RSDoc, RSExtension};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSStruct {
    pub doc: Option<RSDoc>,
    pub name: RSIdentifier,
    // [TODO] Add attributes
    // [TODO] Rather than having extensions, make fields an enum (resolved/unresolved)
    // but this will require changing render functions to return Result rather than
    // String, which is a lot of changes.
    pub extensions: Vec<RSExtension>,
    // [TODO] Rename to fields
    pub properties: Vec<RSProperty>,
}
