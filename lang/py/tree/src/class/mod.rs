use crate::{identifier::PYIdentifier, property::PYProperty, PYDoc, PYExtension};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYClass {
    pub doc: Option<PYDoc>,
    pub name: PYIdentifier,
    pub extensions: Vec<PYExtension>,
    pub properties: Vec<PYProperty>,
}
