use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSUse {
    pub dependency: RSDependencyIdent,
    pub reference: RSUseReference,
}

impl RSUse {
    pub fn new(dependency: RSDependencyIdent, reference: RSUseReference) -> Self {
        Self {
            dependency,
            reference,
        }
    }
}

impl GtlImport for RSUse {}
