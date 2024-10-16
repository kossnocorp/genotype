use crate::{identifier::PYIdentifier, property::PYProperty, PYExtension};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYInterface {
    pub name: PYIdentifier,
    pub extensions: Vec<PYExtension>,
    pub properties: Vec<PYProperty>,
}
