use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYReference {
    pub identifier: PYIdentifier,
    pub forward: bool,
}

impl PYReference {
    pub fn new(identifier: PYIdentifier, forward: bool) -> Self {
        PYReference {
            identifier,
            forward,
        }
    }
}
