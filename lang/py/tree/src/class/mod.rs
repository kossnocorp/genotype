use crate::{identifier::PYIdentifier, property::PYProperty, PYExtension};

mod context;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYClass {
    pub name: PYIdentifier,
    pub extensions: Vec<PYExtension>,
    pub properties: Vec<PYProperty>,
}
