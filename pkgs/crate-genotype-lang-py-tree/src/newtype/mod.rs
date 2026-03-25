use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyNewtype {
    #[visit]
    pub doc: Option<PyDoc>,
    #[visit]
    pub name: PyIdentifier,
    #[visit]
    pub primitive: PyPrimitive,
}
