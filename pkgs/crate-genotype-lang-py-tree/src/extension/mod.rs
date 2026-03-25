use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyExtension {
    #[visit]
    pub reference: PyReference,
}

impl PyExtension {
    pub fn new(reference: PyReference) -> Self {
        PyExtension { reference }
    }
}

impl From<PyReference> for PyExtension {
    fn from(reference: PyReference) -> Self {
        PyExtension::new(reference)
    }
}
