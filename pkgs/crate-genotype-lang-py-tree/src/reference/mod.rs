use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyReference {
    #[visit]
    pub identifier: PyIdentifier,
    #[visit]
    pub arguments: Vec<PyDescriptor>,
    pub forward: bool,
}

impl PyReference {
    pub fn new(identifier: PyIdentifier, forward: bool) -> Self {
        Self::new_with_arguments(identifier, vec![], forward)
    }

    pub fn new_with_arguments(
        identifier: PyIdentifier,
        arguments: Vec<PyDescriptor>,
        forward: bool,
    ) -> Self {
        PyReference {
            identifier,
            arguments,
            forward,
        }
    }
}
