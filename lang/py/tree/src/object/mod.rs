use crate::property::PYProperty;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYObject {
    pub properties: Vec<PYProperty>,
}
