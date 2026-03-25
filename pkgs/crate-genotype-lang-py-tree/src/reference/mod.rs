use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyReference {
    #[visit]
    pub identifier: PyIdentifier,
    pub forward: bool,
}

impl PyReference {
    pub fn new(identifier: PyIdentifier, forward: bool) -> Self {
        PyReference {
            identifier,
            forward,
        }
    }
}
