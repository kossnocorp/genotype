use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYClass {
    pub doc: Option<PYDoc>,
    pub name: PYIdentifier,
    pub extensions: Vec<PYExtension>,
    pub properties: Vec<PYProperty>,
    pub references: Vec<PYIdentifier>,
}
