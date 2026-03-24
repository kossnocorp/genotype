use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYClass {
    #[visit]
    pub doc: Option<PYDoc>,
    #[visit]
    pub name: PYIdentifier,
    #[visit]
    pub extensions: Vec<PYExtension>,
    #[visit]
    pub properties: Vec<PYProperty>,
    #[visit]
    pub references: Vec<PYIdentifier>,
}
