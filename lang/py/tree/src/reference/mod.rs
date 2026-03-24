use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYReference {
    #[visit]
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
