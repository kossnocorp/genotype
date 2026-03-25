use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyProperty {
    #[visit]
    pub doc: Option<PyDoc>,
    #[visit]
    pub name: PyKey,
    #[visit]
    pub descriptor: PyDescriptor,
    pub required: bool,
}
