use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyAlias {
    #[visit]
    pub doc: Option<PyDoc>,
    #[visit]
    pub name: PyIdentifier,
    #[visit]
    pub descriptor: PyDescriptor,
    #[visit]
    pub references: Vec<PyIdentifier>,
}
