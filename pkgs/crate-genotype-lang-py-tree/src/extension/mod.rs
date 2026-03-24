use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYExtension {
    #[visit]
    pub reference: PYReference,
}

impl PYExtension {
    pub fn new(reference: PYReference) -> Self {
        PYExtension { reference }
    }
}

impl From<PYReference> for PYExtension {
    fn from(reference: PYReference) -> Self {
        PYExtension::new(reference)
    }
}
