use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyClass {
    #[visit]
    pub doc: Option<PyDoc>,
    #[visit]
    pub name: PyIdentifier,
    #[visit]
    pub extensions: Vec<PyExtension>,
    #[visit]
    pub properties: Vec<PyProperty>,
    #[visit]
    pub references: Vec<PyIdentifier>,
}
