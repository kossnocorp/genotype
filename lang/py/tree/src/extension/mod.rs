use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct PYExtension {
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
